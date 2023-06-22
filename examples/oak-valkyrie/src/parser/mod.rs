use crate::{ValkyrieLanguage, kind::ValkyrieSyntaxKind};
use oak_core::{
    GreenNode,
    parser::{Associativity, ParserState, Pratt, binary, unary},
    source::Source,
};

#[allow(dead_code)]
type State<'a, S> = ParserState<'a, ValkyrieLanguage, S>;

mod parse;

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
            ValkyrieSyntaxKind::OrOr => (14, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::AndAnd => (15, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Bitwise
            ValkyrieSyntaxKind::Or => (20, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::Caret => (21, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::And => (22, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Comparison
            ValkyrieSyntaxKind::EqEq | ValkyrieSyntaxKind::Ne => (25, Associativity::None, ValkyrieSyntaxKind::BinaryExpression),
            ValkyrieSyntaxKind::Lt | ValkyrieSyntaxKind::Gt | ValkyrieSyntaxKind::Le | ValkyrieSyntaxKind::Ge => (30, Associativity::None, ValkyrieSyntaxKind::BinaryExpression),

            // Shift
            ValkyrieSyntaxKind::Shl | ValkyrieSyntaxKind::Shr => (35, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Additive
            ValkyrieSyntaxKind::Plus | ValkyrieSyntaxKind::Minus => (40, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Multiplicative
            ValkyrieSyntaxKind::Star | ValkyrieSyntaxKind::Slash | ValkyrieSyntaxKind::Percent => (50, Associativity::Left, ValkyrieSyntaxKind::BinaryExpression),

            // Postfix / Access
            ValkyrieSyntaxKind::LeftParen => (70, Associativity::Left, ValkyrieSyntaxKind::CallExpression),
            ValkyrieSyntaxKind::Dot => (70, Associativity::Left, ValkyrieSyntaxKind::FieldExpression),
            ValkyrieSyntaxKind::LeftBracket => (70, Associativity::Left, ValkyrieSyntaxKind::IndexExpression),
            ValkyrieSyntaxKind::LeftBrace => (70, Associativity::Left, ValkyrieSyntaxKind::ApplyBlock),

            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        let cp = state.checkpoint();
        let node = match kind {
            ValkyrieSyntaxKind::LeftParen => {
                let cp_inner = state.checkpoint();
                state.push_child(left);
                state.expect(ValkyrieSyntaxKind::LeftParen).ok();
                while let Some(t) = state.current() {
                    if t.kind == ValkyrieSyntaxKind::RightParen {
                        break;
                    }
                    let arg = self.parse_expression_internal(state, 0);
                    state.push_child(arg);
                    if state.at(ValkyrieSyntaxKind::Comma) {
                        state.expect(ValkyrieSyntaxKind::Comma).ok();
                    }
                }
                state.expect(ValkyrieSyntaxKind::RightParen).ok();
                state.finish_at(cp_inner, ValkyrieSyntaxKind::CallExpression)
            }
            ValkyrieSyntaxKind::LeftBracket => {
                let cp_inner = state.checkpoint();
                state.push_child(left);
                state.expect(ValkyrieSyntaxKind::LeftBracket).ok();
                let index = self.parse_expression_internal(state, 0);
                state.push_child(index);
                state.expect(ValkyrieSyntaxKind::RightBracket).ok();
                state.finish_at(cp_inner, ValkyrieSyntaxKind::IndexExpression)
            }
            ValkyrieSyntaxKind::LeftBrace => {
                let cp_inner = state.checkpoint();
                state.push_child(left);
                self.parse_block_expr_node(state).ok();
                state.finish_at(cp_inner, ValkyrieSyntaxKind::ApplyBlock)
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

        let cp = state.checkpoint();
        unary(state, kind, prec, result_kind, |s, p| self.parse_expression_internal(s, p))
    }
}

impl<'config> oak_core::parser::Parser<ValkyrieLanguage> for ValkyrieParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::parser::ParseCache<ValkyrieLanguage>) -> oak_core::ParseOutput<'a, ValkyrieLanguage> {
        let lexer = crate::lexer::ValkyrieLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| Ok(self.parse_root_internal(state)))
    }
}
