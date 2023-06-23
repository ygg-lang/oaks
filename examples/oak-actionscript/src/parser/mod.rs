pub mod element_type;

pub use element_type::ActionScriptElementType;

use crate::{language::ActionScriptLanguage, lexer::ActionScriptLexer};
use oak_core::{
    GreenNode,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, ActionScriptLanguage, S>;

pub struct ActionScriptParser<'config> {
    pub(crate) config: &'config ActionScriptLanguage,
}

impl<'config> Pratt<ActionScriptLanguage> for ActionScriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ActionScriptLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(crate::lexer::ActionScriptTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::IdentifierExpression)
            }
            Some(k) if k.is_literal() => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::LiteralExpression)
            }
            Some(crate::lexer::ActionScriptTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::ParenthesizedExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ActionScriptLanguage> {
        use crate::{lexer::ActionScriptTokenType as TT, parser::element_type::ActionScriptElementType as ET};
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            TT::Minus | TT::LogicalNot | TT::BitwiseNot => unary(state, kind, 13, ET::UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ActionScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ActionScriptLanguage>> {
        use crate::{lexer::ActionScriptTokenType as TT, parser::element_type::ActionScriptElementType as ET};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            TT::Equal
            | TT::PlusAssign
            | TT::MinusAssign
            | TT::StarAssign
            | TT::SlashAssign
            | TT::PercentAssign
            | TT::LeftShiftAssign
            | TT::RightShiftAssign
            | TT::UnsignedRightShiftAssign
            | TT::BitwiseAndAssign
            | TT::BitwiseOrAssign
            | TT::BitwiseXorAssign => (1, Associativity::Right),
            TT::LogicalOr => (3, Associativity::Left),
            TT::LogicalAnd => (4, Associativity::Left),
            TT::EqualEqual | TT::NotEqual | TT::EqualEqualEqual | TT::NotEqualEqual => (5, Associativity::Left),
            TT::LessThan | TT::LessEqual | TT::GreaterThan | TT::GreaterEqual | TT::Is | TT::Instanceof => (6, Associativity::Left),
            TT::BitwiseOr => (7, Associativity::Left),
            TT::BitwiseXor => (8, Associativity::Left),
            TT::BitwiseAnd => (9, Associativity::Left),
            TT::LeftShift | TT::RightShift | TT::UnsignedRightShift => (10, Associativity::Left),
            TT::Plus | TT::Minus => (11, Associativity::Left),
            TT::Star | TT::Slash | TT::Percent => (12, Associativity::Left),
            TT::LeftParen | TT::LeftBracket | TT::Dot => (14, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            TT::LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::LeftParen).ok();
                if !state.at(TT::RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(TT::Comma) {
                            break;
                        }
                    }
                }
                state.expect(TT::RightParen).ok();
                Some(state.finish_at(cp, ET::CallExpression))
            }
            TT::LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                state.expect(TT::RightBracket).ok();
                Some(state.finish_at(cp, ET::IndexExpression))
            }
            TT::Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::Dot).ok();
                state.expect(TT::Identifier).ok();
                Some(state.finish_at(cp, ET::FieldExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, ET::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> ActionScriptParser<'config> {
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<ActionScriptLanguage> for ActionScriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ActionScriptLanguage>) -> ParseOutput<'a, ActionScriptLanguage> {
        let lexer = ActionScriptLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_source_file(state))
    }
}
