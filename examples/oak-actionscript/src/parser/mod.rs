pub mod element_type;

pub use element_type::ActionScriptElementType;

use crate::{language::ActionScriptLanguage, lexer::ActionScriptLexer};
use oak_core::{
    GreenNode,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

mod parse;

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
                state.finish_at(cp, ActionScriptElementType::IdentifierExpression.into())
            }
            Some(k) if k.is_literal() => {
                state.bump();
                state.finish_at(cp, ActionScriptElementType::LiteralExpression.into())
            }
            Some(crate::lexer::ActionScriptTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
                state.finish_at(cp, ActionScriptElementType::ParenthesizedExpression.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, ActionScriptElementType::Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ActionScriptLanguage> {
        use crate::{lexer::ActionScriptTokenType::*, parser::ActionScriptElementType::*};
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Minus | LogicalNot | BitwiseNot => unary(state, kind, 13, UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ActionScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ActionScriptLanguage>> {
        use crate::{lexer::ActionScriptTokenType::*, parser::ActionScriptElementType::*};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Equal | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | LeftShiftAssign | RightShiftAssign | UnsignedRightShiftAssign | BitwiseAndAssign | BitwiseOrAssign | BitwiseXorAssign => (1, Associativity::Right),
            LogicalOr => (3, Associativity::Left),
            LogicalAnd => (4, Associativity::Left),
            EqualEqual | NotEqual | EqualEqualEqual | NotEqualEqual => (5, Associativity::Left),
            LessThan | LessEqual | GreaterThan | GreaterEqual | Is | Instanceof => (6, Associativity::Left),
            BitwiseOr => (7, Associativity::Left),
            BitwiseXor => (8, Associativity::Left),
            BitwiseAnd => (9, Associativity::Left),
            LeftShift | RightShift | UnsignedRightShift => (10, Associativity::Left),
            Plus | Minus => (11, Associativity::Left),
            Star | Slash | Percent => (12, Associativity::Left),
            LeftParen | LeftBracket | Dot => (14, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                if !state.at(RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(Comma) {
                            break;
                        }
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CallExpression.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, IndexExpression.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
                Some(state.finish_at(cp, FieldExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
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
