//! Matlab Pratt expression parser (arithmetic / calls / arrays).

/// Element kinds.
pub mod element_type;

use crate::{
    language::MatlabLanguage,
    lexer::{MatlabLexer, token_type::MatlabTokenType},
    parser::element_type::MatlabElementType,
};
use oak_core::{
    parser::{OperatorInfo, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, postfix, unary},
    source::{Source, TextEdit},
    tree::GreenNode,
};

type State<'a, S> = ParserState<'a, MatlabLanguage, S>;

/// MATLAB parser.
#[derive(Debug, Clone)]
pub struct MatlabParser<'config> {
    /// Language configuration.
    config: &'config MatlabLanguage,
}

impl<'config> MatlabParser<'config> {
    /// Creates a new parser.
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<MatlabLanguage> for MatlabParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MatlabLanguage>) -> ParseOutput<'a, MatlabLanguage> {
        let lexer = MatlabLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() && state.not_at(MatlabTokenType::Eof) {
                self.parse_expression(state);
                if state.at(MatlabTokenType::Semicolon) {
                    state.bump();
                }
            }
            Ok(state.finish_at(checkpoint, MatlabElementType::Root))
        })
    }
}

impl<'config> MatlabParser<'config> {
    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        self.parse_pratt(state, 0);
    }

    fn parse_pratt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, MatlabLanguage> {
        PrattParser::new(self.clone()).parse_expr(state, min_precedence)
    }

    fn parse_call_args<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // (
        while state.not_at(MatlabTokenType::RightParen) && state.not_at_end() {
            self.parse_expression(state);
            if state.at(MatlabTokenType::Comma) {
                state.bump();
            }
        }
        if state.at(MatlabTokenType::RightParen) {
            state.bump();
        }
        state.finish_at(checkpoint, MatlabElementType::Arguments);
    }

    fn parse_array<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, MatlabLanguage> {
        let checkpoint = state.checkpoint();
        state.bump(); // [
        while state.not_at(MatlabTokenType::RightBracket) && state.not_at_end() {
            self.parse_expression(state);
            if state.at(MatlabTokenType::Comma) || state.at(MatlabTokenType::Semicolon) {
                state.bump();
            }
        }
        if state.at(MatlabTokenType::RightBracket) {
            state.bump();
        }
        state.finish_at(checkpoint, MatlabElementType::Array)
    }
}

impl<'config> Pratt<MatlabLanguage> for MatlabParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, MatlabLanguage> {
        let checkpoint = state.checkpoint();

        if state.at(MatlabTokenType::Identifier) {
            state.bump();
            if state.at(MatlabTokenType::LeftParen) {
                while state.at(MatlabTokenType::LeftParen) {
                    self.parse_call_args(state);
                }
                state.finish_at(checkpoint, MatlabElementType::Call)
            }
            else {
                state.finish_at(checkpoint, MatlabElementType::Symbol)
            }
        }
        else if state.at(MatlabTokenType::Number) || state.at(MatlabTokenType::String) || state.at(MatlabTokenType::Character) {
            state.bump();
            state.finish_at(checkpoint, MatlabElementType::Literal)
        }
        else if state.at(MatlabTokenType::LeftBracket) {
            self.parse_array(state)
        }
        else if state.at(MatlabTokenType::LeftParen) {
            state.bump();
            self.parse_expression(state);
            if state.at(MatlabTokenType::RightParen) {
                state.bump();
            }
            state.finish_at(checkpoint, MatlabElementType::Expression)
        }
        else {
            state.bump();
            state.finish_at(checkpoint, MatlabElementType::Error)
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, MatlabLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };
        let info = match kind {
            MatlabTokenType::Minus | MatlabTokenType::Plus | MatlabTokenType::Not => Some(OperatorInfo::right(150)),
            _ => None,
        };
        if let Some(info) = info { unary(state, kind, info.precedence, MatlabElementType::PrefixExpr, |s, p| self.parse_pratt(s, p)) } else { self.primary(state) }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, MatlabLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, MatlabLanguage>> {
        let kind = state.peek_kind()?;

        let postfix_info = match kind {
            MatlabTokenType::Transpose | MatlabTokenType::DotTranspose => Some(OperatorInfo::left(160)),
            _ => None,
        };
        if let Some(info) = postfix_info {
            if info.precedence < min_precedence {
                return None;
            }
            return Some(postfix(state, left, kind, MatlabElementType::PostfixExpr));
        }

        let info = match kind {
            MatlabTokenType::Assign => Some(OperatorInfo::right(20)),
            MatlabTokenType::OrOr => Some(OperatorInfo::left(40)),
            MatlabTokenType::AndAnd => Some(OperatorInfo::left(50)),
            MatlabTokenType::Equal | MatlabTokenType::NotEqual | MatlabTokenType::Less | MatlabTokenType::Greater | MatlabTokenType::LessEqual | MatlabTokenType::GreaterEqual => Some(OperatorInfo::none(60)),
            MatlabTokenType::Colon => Some(OperatorInfo::left(70)),
            MatlabTokenType::Plus | MatlabTokenType::Minus => Some(OperatorInfo::left(80)),
            MatlabTokenType::Times | MatlabTokenType::Divide | MatlabTokenType::LeftDivide | MatlabTokenType::DotTimes | MatlabTokenType::DotDivide | MatlabTokenType::DotLeftDivide => Some(OperatorInfo::left(90)),
            MatlabTokenType::Power | MatlabTokenType::DotPower => Some(OperatorInfo::right(120)),
            _ => None,
        }?;

        if info.precedence < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, info.precedence, info.associativity, MatlabElementType::BinaryExpr, |s, p| self.parse_pratt(s, p)))
    }
}
