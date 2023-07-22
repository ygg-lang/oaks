//! Expression nodes for MATLAB.

use crate::{ast::root_nodes::{Identifier, Span}, lexer::token_type::MatlabTokenType};

/// A MATLAB expression (owned).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Identifier / `end` / lone `:`.
    Symbol(Identifier),
    /// Number / string / character literal.
    Literal {
        /// Raw literal text.
        value: String,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Array `[…]` with row groups.
    Array {
        /// Rows of elements.
        rows: Vec<Vec<Expression>>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Call / indexing `f(…)` / `A(…)`.
    Call {
        /// Head expression.
        head: Box<Expression>,
        /// Arguments / indices.
        arguments: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Binary / infix operator.
    Binary(Box<BinaryExpr>),
    /// Prefix operator.
    Prefix(Box<UnaryExpr>),
    /// Postfix operator (transpose).
    Postfix(Box<UnaryExpr>),
    /// Parenthesized expression.
    Grouped {
        /// Inner expression.
        expression: Box<Expression>,
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
    pub operator: MatlabTokenType,
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
    pub operator: MatlabTokenType,
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
            Self::Literal { span, .. } | Self::Array { span, .. } | Self::Call { span, .. } | Self::Grouped { span, .. } => span.clone(),
            Self::Binary(b) => b.span.clone(),
            Self::Prefix(u) | Self::Postfix(u) => u.span.clone(),
        }
    }
}
