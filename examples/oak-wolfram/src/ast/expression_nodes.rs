//! Expression nodes for Wolfram Language.

use crate::{ast::root_nodes::{Identifier, Span}, lexer::token_type::WolframTokenType};

/// A Wolfram expression (owned).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Symbol / keyword head finished as a name.
    Symbol(Identifier),
    /// Integer, real, or string literal.
    Literal {
        /// Raw literal text (quotes kept for strings).
        value: String,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// List `{…}`.
    List {
        /// Elements.
        elements: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Call `f[…]` / `expr[…]`.
    Call {
        /// Head expression.
        head: Box<Expression>,
        /// Flattened arguments (one bracket group). Nested `f[a][b]` is nested `Call`.
        arguments: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Part `expr[[…]]`.
    Part {
        /// Indexed expression.
        expression: Box<Expression>,
        /// Indices.
        indices: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Binary / infix operator.
    Binary(Box<BinaryExpr>),
    /// Prefix operator.
    Prefix(Box<UnaryExpr>),
    /// Postfix operator (`&`, `!`).
    Postfix(Box<UnaryExpr>),
    /// Blank `_` / `__` / `___`, optionally typed.
    Blank {
        /// Underscore token kind.
        kind: WolframTokenType,
        /// Optional typed head (`_Integer`).
        head: Option<Box<Expression>>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Named pattern `x_`.
    Pattern {
        /// Pattern name.
        name: Box<Expression>,
        /// Underscore token kind.
        blank: WolframTokenType,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Parenthesized expression.
    Grouped {
        /// Inner expression.
        expression: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Recovery / error node.
    Error {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}

/// Binary operator application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryExpr {
    /// Operator token kind.
    pub operator: WolframTokenType,
    /// Left operand.
    pub lhs: Expression,
    /// Right operand.
    pub rhs: Expression,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// Unary (prefix or postfix) operator application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryExpr {
    /// Operator token kind.
    pub operator: WolframTokenType,
    /// Operand.
    pub operand: Expression,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

impl Expression {
    /// Source span of this expression.
    pub fn span(&self) -> Span {
        match self {
            Self::Symbol(id) => id.span.clone(),
            Self::Literal { span, .. }
            | Self::List { span, .. }
            | Self::Call { span, .. }
            | Self::Part { span, .. }
            | Self::Blank { span, .. }
            | Self::Pattern { span, .. }
            | Self::Grouped { span, .. }
            | Self::Error { span } => span.clone(),
            Self::Binary(b) => b.span.clone(),
            Self::Prefix(u) | Self::Postfix(u) => u.span.clone(),
        }
    }
}
