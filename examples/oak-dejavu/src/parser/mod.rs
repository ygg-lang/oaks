#![doc = include_str!("readme.md")]
pub mod element_type;
pub use crate::parser::element_type::DejavuElementType;

use crate::{
    language::DejavuLanguage,
    lexer::{DejavuKeywords, token_type::DejavuSyntaxKind},
};
use oak_core::{
    GreenNode,
    parser::{Associativity, ParserState, Pratt, binary, unary},
    source::Source,
};

#[allow(unused_imports)]
#[allow(dead_code)]
type State<'a, S> = ParserState<'a, DejavuLanguage, S>;

mod parse_top_level;

/// A parser for the Dejavu programming language.
#[derive(Clone)]
pub struct DejavuParser<'config> {
    /// Language configuration
    config: &'config DejavuLanguage,
}

impl<'config> DejavuParser<'config> {}

impl<'config> DejavuParser<'config> {
    /// Creates a new Dejavu parser.
    pub fn new(config: &'config DejavuLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<DejavuLanguage> for DejavuParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, DejavuLanguage, S>) -> &'a GreenNode<'a, DejavuLanguage> {
        let cp = state.checkpoint();
        self.parse_primary(state).unwrap_or_else(|_| {
            state.restore(cp);
            state.bump();
            state.finish_at(cp, DejavuSyntaxKind::Error)
        })
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, DejavuLanguage, S>, left: &'a GreenNode<'a, DejavuLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, DejavuLanguage>> {
        self.skip_trivia(state);
        let t = state.current()?;
        let kind = t.kind;

        let (prec, assoc, result_kind) = match kind {
            // Assignment
            DejavuSyntaxKind::Eq
            | DejavuSyntaxKind::PlusEq
            | DejavuSyntaxKind::MinusEq
            | DejavuSyntaxKind::StarEq
            | DejavuSyntaxKind::SlashEq
            | DejavuSyntaxKind::PercentEq
            | DejavuSyntaxKind::CaretEq
            | DejavuSyntaxKind::AndEq
            | DejavuSyntaxKind::OrEq
            | DejavuSyntaxKind::ShlEq
            | DejavuSyntaxKind::ShrEq => (10, Associativity::Right, DejavuSyntaxKind::BinaryExpression),

            // Logical
            DejavuSyntaxKind::PipeGreater => (12, Associativity::Left, DejavuSyntaxKind::BinaryExpression),
            DejavuSyntaxKind::OrOr => (14, Associativity::Left, DejavuSyntaxKind::BinaryExpression),
            DejavuSyntaxKind::AndAnd => (15, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            // Bitwise
            DejavuSyntaxKind::Or => (20, Associativity::Left, DejavuSyntaxKind::BinaryExpression),
            DejavuSyntaxKind::Caret => (21, Associativity::Left, DejavuSyntaxKind::BinaryExpression),
            DejavuSyntaxKind::And => (22, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            // Comparison
            DejavuSyntaxKind::EqEq | DejavuSyntaxKind::NotEq | DejavuSyntaxKind::Keyword(DejavuKeywords::Is) | DejavuSyntaxKind::Keyword(DejavuKeywords::In) => (80, Associativity::None, DejavuSyntaxKind::BinaryExpression),
            DejavuSyntaxKind::LessThan | DejavuSyntaxKind::GreaterThan | DejavuSyntaxKind::LessEq | DejavuSyntaxKind::GreaterEq => (90, Associativity::None, DejavuSyntaxKind::BinaryExpression),

            // Shift
            DejavuSyntaxKind::LeftShift | DejavuSyntaxKind::RightShift => (35, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            // Additive
            DejavuSyntaxKind::Plus | DejavuSyntaxKind::Minus => (40, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            // Multiplicative
            DejavuSyntaxKind::Star | DejavuSyntaxKind::Slash | DejavuSyntaxKind::Percent => (50, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            // Postfix / Access
            DejavuSyntaxKind::LeftParen => (100, Associativity::Left, DejavuSyntaxKind::CallExpression),
            DejavuSyntaxKind::Dot => (100, Associativity::Left, DejavuSyntaxKind::FieldExpression),
            DejavuSyntaxKind::LeftBracket => (100, Associativity::Left, DejavuSyntaxKind::IndexExpression),
            DejavuSyntaxKind::LeftBrace => (5, Associativity::Left, DejavuSyntaxKind::ObjectExpression),
            DejavuSyntaxKind::Colon => (150, Associativity::Left, DejavuSyntaxKind::BinaryExpression),

            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        let _cp = state.checkpoint();
        let node = match kind {
            DejavuSyntaxKind::LeftParen => {
                let cp = state.checkpoint_before(left);
                state.expect(DejavuSyntaxKind::LeftParen).ok();
                while let Some(t) = state.current() {
                    if t.kind == DejavuSyntaxKind::RightParen {
                        break;
                    }
                    self.parse_expression_internal(state, 0);
                    if state.at(DejavuSyntaxKind::Comma) {
                        state.expect(DejavuSyntaxKind::Comma).ok();
                    }
                    self.skip_trivia(state);
                }
                state.expect(DejavuSyntaxKind::RightParen).ok();
                self.skip_trivia(state);
                if state.at(DejavuSyntaxKind::LeftBrace) {
                    self.parse_block_expr_node(state).ok();
                    state.finish_at(cp, DejavuSyntaxKind::ObjectExpression)
                }
                else {
                    state.finish_at(cp, DejavuSyntaxKind::CallExpression)
                }
            }
            DejavuSyntaxKind::LeftBracket => {
                let cp = state.checkpoint_before(left);
                state.expect(DejavuSyntaxKind::LeftBracket).ok();
                self.parse_expression_internal(state, 0);
                state.expect(DejavuSyntaxKind::RightBracket).ok();
                self.skip_trivia(state);
                if state.at(DejavuSyntaxKind::LeftBrace) {
                    self.parse_block_expr_node(state).ok();
                    state.finish_at(cp, DejavuSyntaxKind::ObjectExpression)
                }
                else {
                    state.finish_at(cp, DejavuSyntaxKind::IndexExpression)
                }
            }
            DejavuSyntaxKind::LeftBrace => {
                let cp = state.checkpoint_before(left);
                self.parse_block_expr_node(state).ok();
                state.finish_at(cp, DejavuSyntaxKind::ObjectExpression)
            }
            _ => binary(state, left, kind, prec, assoc, result_kind, |s, p| self.parse_expression_internal(s, p)),
        };
        Some(node)
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, DejavuLanguage, S>) -> &'a GreenNode<'a, DejavuLanguage> {
        self.skip_trivia(state);
        let t = match state.current() {
            Some(t) => t,
            None => return self.primary(state),
        };

        let kind = t.kind;
        let (prec, result_kind) = match kind {
            DejavuSyntaxKind::Not | DejavuSyntaxKind::Minus | DejavuSyntaxKind::Plus | DejavuSyntaxKind::Star | DejavuSyntaxKind::And => (60, DejavuSyntaxKind::UnaryExpression),
            _ => return self.primary(state),
        };

        let _cp = state.checkpoint();
        unary(state, kind, prec, result_kind, |s, p| self.parse_expression_internal(s, p))
    }
}

impl<'config> oak_core::parser::Parser<DejavuLanguage> for DejavuParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::parser::ParseCache<DejavuLanguage>) -> oak_core::ParseOutput<'a, DejavuLanguage> {
        let lexer = crate::lexer::DejavuLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| Ok(self.parse_root_internal(state)))
    }
}
