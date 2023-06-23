pub mod element_type;

use crate::{
    language::WolframLanguage,
    lexer::{WolframLexer, token_type::WolframTokenType},
    parser::element_type::WolframElementType,
};
use oak_core::{
    parser::{Associativity, OperatorInfo, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, postfix, unary},
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, WolframLanguage, S>;

/// Wolfram Parser
#[derive(Debug, Clone)]
pub struct WolframParser<'config> {
    config: &'config WolframLanguage,
}

impl<'config> WolframParser<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WolframLanguage> for WolframParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<WolframLanguage>) -> ParseOutput<'a, WolframLanguage> {
        let lexer = WolframLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_expression(state);
            }

            Ok(state.finish_at(checkpoint, WolframElementType::Root))
        })
    }
}

impl<'config> WolframParser<'config> {
    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        self.parse_pratt(state, 0);
    }

    fn parse_pratt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, WolframLanguage> {
        PrattParser::new(self.clone()).parse_expr(state, min_precedence)
    }

    fn parse_arguments<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // [

        while state.not_at(WolframTokenType::RightBracket) && state.not_at_end() {
            self.parse_expression(state);
            if state.at(WolframTokenType::Comma) {
                state.bump();
            }
        }

        if state.at(WolframTokenType::RightBracket) {
            state.bump();
        }
        state.finish_at(checkpoint, WolframElementType::Arguments);
    }

    fn parse_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // {

        while state.not_at(WolframTokenType::RightBrace) && state.not_at_end() {
            self.parse_expression(state);
            if state.at(WolframTokenType::Comma) {
                state.bump();
            }
        }

        if state.at(WolframTokenType::RightBrace) {
            state.bump();
        }
        state.finish_at(checkpoint, WolframElementType::List);
    }
}

impl<'config> Pratt<WolframLanguage> for WolframParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, WolframLanguage> {
        let checkpoint = state.checkpoint();

        if state.at(WolframTokenType::Identifier) {
            state.bump();
            // 检查是否是函数调用 f[...]
            while state.at(WolframTokenType::LeftBracket) {
                self.parse_arguments(state);
                state.finish_at(checkpoint, WolframElementType::Call);
            }
            if state.checkpoint() == checkpoint { state.finish_at(checkpoint, WolframElementType::Symbol) } else { state.finish_at(checkpoint, WolframElementType::Call) }
        }
        else if state.at(WolframTokenType::Integer) || state.at(WolframTokenType::Real) || state.at(WolframTokenType::String) {
            state.bump();
            state.finish_at(checkpoint, WolframElementType::Literal)
        }
        else if state.at(WolframTokenType::LeftBrace) {
            self.parse_list(state);
            state.finish_at(checkpoint, WolframElementType::List)
        }
        else if state.at(WolframTokenType::Slot) || state.at(WolframTokenType::SlotSequence) {
            state.bump();
            state.finish_at(checkpoint, WolframElementType::Symbol)
        }
        else if state.at(WolframTokenType::LeftParen) {
            state.bump();
            self.parse_expression(state);
            if state.at(WolframTokenType::RightParen) {
                state.bump();
            }
            state.finish_at(checkpoint, WolframElementType::Expression)
        }
        else {
            // 容错处理
            state.bump();
            state.finish_at(checkpoint, WolframElementType::Error)
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, WolframLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        let info = match kind {
            WolframTokenType::Minus => Some(OperatorInfo::right(150)),     // Unary minus
            WolframTokenType::Factorial => Some(OperatorInfo::right(150)), // ! (Not)
            _ => None,
        };

        if let Some(info) = info { unary(state, kind, info.precedence, WolframElementType::PrefixExpr, |s, p| self.parse_pratt(s, p)) } else { self.primary(state) }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, WolframLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, WolframLanguage>> {
        let kind = state.peek_kind()?;

        // 后缀运算符
        let postfix_info = match kind {
            WolframTokenType::Ampersand => Some(OperatorInfo::left(10)),  // body &
            WolframTokenType::Factorial => Some(OperatorInfo::left(160)), // x!
            _ => None,
        };

        if let Some(info) = postfix_info {
            if info.precedence < min_precedence {
                return None;
            }
            return Some(postfix(state, left, kind, WolframElementType::PostfixExpr));
        }

        // 二元/中缀运算符
        let info = match kind {
            WolframTokenType::Assign | WolframTokenType::Set | WolframTokenType::SetDelayed => Some(OperatorInfo::right(20)),
            WolframTokenType::Rule | WolframTokenType::RuleDelayed | WolframTokenType::Arrow => Some(OperatorInfo::right(30)),
            WolframTokenType::SlashSlash => Some(OperatorInfo::left(40)), // x // f
            WolframTokenType::Or => Some(OperatorInfo::left(50)),
            WolframTokenType::And => Some(OperatorInfo::left(60)),
            WolframTokenType::Equal | WolframTokenType::NotEqual | WolframTokenType::Less | WolframTokenType::Greater | WolframTokenType::LessEqual | WolframTokenType::GreaterEqual => Some(OperatorInfo::none(70)),
            WolframTokenType::Plus | WolframTokenType::Minus => Some(OperatorInfo::left(80)),
            WolframTokenType::Times | WolframTokenType::Divide => Some(OperatorInfo::left(90)),
            WolframTokenType::At => Some(OperatorInfo::right(100)),                 // f @ x
            WolframTokenType::MapOperator => Some(OperatorInfo::right(110)),        // f /@ list
            WolframTokenType::ApplyOperator => Some(OperatorInfo::right(110)),      // f @@ expr
            WolframTokenType::ApplyLevelOperator => Some(OperatorInfo::right(110)), // f @@@ expr
            WolframTokenType::MapAllOperator => Some(OperatorInfo::right(110)),     // f //@ list
            WolframTokenType::Power => Some(OperatorInfo::right(120)),
            _ => None,
        }?;

        if info.precedence < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, info.precedence, info.associativity, WolframElementType::BinaryExpr, |s, p| self.parse_pratt(s, p)))
    }
}
