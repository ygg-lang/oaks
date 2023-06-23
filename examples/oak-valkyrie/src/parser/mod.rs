#![doc = include_str!("readme.md")]
pub mod element_type;
pub use crate::parser::element_type::ValkyrieElementType;

use crate::{
    language::ValkyrieLanguage,
    lexer::{ValkyrieKeywords, token_type::ValkyrieSyntaxKind},
};
use oak_core::{
    GreenNode,
    parser::{Associativity, ParserState, Pratt, binary, unary},
    source::Source,
};

#[allow(unused_imports)]
#[allow(dead_code)]
type State<'a, S> = ParserState<'a, ValkyrieLanguage, S>;

mod parse_top_level;

/// A parser for the Valkyrie programming language.
#[derive(Clone)]
pub struct ValkyrieParser<'config> {
    /// Language configuration
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieParser<'config> {}

impl<'config> ValkyrieParser<'config> {
    /// Creates a new Valkyrie parser.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<ValkyrieLanguage> for ValkyrieParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, ValkyrieLanguage, S>) -> &'a GreenNode<'a, ValkyrieLanguage> {
        let cp = state.checkpoint();
        self.parse_primary(state).unwrap_or_else(|_| {
            state.restore(cp);
            state.bump();
            state.finish_at(cp, ValkyrieSyntaxKind::Error)
        })
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, ValkyrieLanguage, S>, left: &'a GreenNode<'a, ValkyrieLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ValkyrieLanguage>> {
        self.skip_trivia(state);
        let t = state.current()?;
        let kind = t.kind;

        let (prec, assoc, result_kind) = match kind {
            // Assignment
            ValkyrieSyntaxKind::Eq
            | ValkyrieSyntaxKind::PlusEq
            | ValkyrieSyntaxKind::MinusEq
            | ValkyrieSyntaxKind::StarEq
            | ValkyrieSyntaxKind::SlashEq
            | ValkyrieSyntaxKind::PercentEq
            | ValkyrieSyntaxKind::CaretEq
            | ValkyrieSyntaxKind::AndEq
            | ValkyrieSyntaxKind::OrEq
            | ValkyrieSyntaxKind::ShlEq
            | ValkyrieSyntaxKind::ShrEq => (10, Associativity::Right, ValkyrieSyntaxKind::BinaryExpression),

            // Logical
            ValkyrieSyntaxKind::PipeGreater => (12, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::OrOr => (14, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::AndAnd => (15, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Bitwise
            ValkyrieSyntaxKind::Or => (20, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::Caret => (21, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::And => (22, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Comparison
            ValkyrieSyntaxKind::EqEq | ValkyrieSyntaxKind::NotEq | ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Is) | ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::In) => (80, Associativity::None, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::LessThan | ValkyrieSyntaxKind::GreaterThan | ValkyrieSyntaxKind::LessEq | ValkyrieSyntaxKind::GreaterEq => (90, Associativity::None, ValkyrieSyntaxKind::BinaryExpression),

            // Shift
            ValkyrieSyntaxKind::LeftShift | ValkyrieSyntaxKind::RightShift => (35, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Additive
            ValkyrieSyntaxKind::Plus | ValkyrieSyntaxKind::Minus => (40, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Multiplicative
            ValkyrieSyntaxKind::Star | ValkyrieSyntaxKind::Slash | ValkyrieSyntaxKind::Percent => (50, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Postfix / Access
            ValkyrieSyntaxKind::LeftParen => (100, Associativity::Left, ValkyrieSyntaxKind::CallExpression),
            ValkyrieSyntaxKind::Dot => (100, Associativity::Left, ValkyrieSyntaxKind::FieldExpression),
            ValkyrieSyntaxKind::LeftBracket => (100, Associativity::Left, ValkyrieSyntaxKind::IndexExpression),
            ValkyrieSyntaxKind::LeftBrace => (5, Associativity::Left, ValkyrieSyntaxKind::ObjectExpression),
            ValkyrieSyntaxKind::Colon => (150, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        let _cp = state.checkpoint();
        let node = match kind {
            ValkyrieSyntaxKind::LeftParen => {
                let cp = state.checkpoint_before(left);
                state.expect(ValkyrieSyntaxKind::LeftParen).ok();
                while let Some(t) = state.current() {
                    if t.kind == ValkyrieSyntaxKind::RightParen {
                        break;
                    }
                    self.parse_expression_internal(state, 0);
                    if state.at(ValkyrieSyntaxKind::Comma) {
                        state.expect(ValkyrieSyntaxKind::Comma).ok();
                    }
                    self.skip_trivia(state);
                }
                state.expect(ValkyrieSyntaxKind::RightParen).ok();
                self.skip_trivia(state);
                if state.at(ValkyrieSyntaxKind::LeftBrace) {
                    self.parse_block_expr_node(state).ok();
                    state.finish_at(cp, ValkyrieSyntaxKind::ObjectExpression)
                }
                else {
                    state.finish_at(cp, ValkyrieSyntaxKind::CallExpression)
                }
            }
            ValkyrieSyntaxKind::LeftBracket => {
                let cp = state.checkpoint_before(left);
                state.expect(ValkyrieSyntaxKind::LeftBracket).ok();
                self.parse_expression_internal(state, 0);
                state.expect(ValkyrieSyntaxKind::RightBracket).ok();
                self.skip_trivia(state);
                if state.at(ValkyrieSyntaxKind::LeftBrace) {
                    self.parse_block_expr_node(state).ok();
                    state.finish_at(cp, ValkyrieSyntaxKind::ObjectExpression)
                }
                else {
                    state.finish_at(cp, ValkyrieSyntaxKind::IndexExpression)
                }
            }
            ValkyrieSyntaxKind::LeftBrace => {
                let cp = state.checkpoint_before(left);
                self.parse_block_expr_node(state).ok();
                state.finish_at(cp, ValkyrieSyntaxKind::ObjectExpression)
            }
            _ => binary(state, left, kind, prec, assoc, result_kind, |s, p| self.parse_expression_internal(s, p)),
        };
        Some(node)
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, ValkyrieLanguage, S>) -> &'a GreenNode<'a, ValkyrieLanguage> {
        self.skip_trivia(state);
        let t = match state.current() {
            Some(t) => t,
            None => return self.primary(state),
        };

        let kind = t.kind;
        let (prec, result_kind) = match kind {
            ValkyrieSyntaxKind::Not | ValkyrieSyntaxKind::Minus | ValkyrieSyntaxKind::Plus | ValkyrieSyntaxKind::Star | ValkyrieSyntaxKind::And => (60, ValkyrieSyntaxKind::UnaryExpression),
            _ => return self.primary(state),
        };

        let _cp = state.checkpoint();
        unary(state, kind, prec, result_kind, |s, p| self.parse_expression_internal(s, p))
    }
}

impl<'config> oak_core::parser::Parser<ValkyrieLanguage> for ValkyrieParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::parser::ParseCache<ValkyrieLanguage>) -> oak_core::ParseOutput<'a, ValkyrieLanguage> {
        let lexer = crate::lexer::ValkyrieLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| Ok(self.parse_root_internal(state)))
    }
}
