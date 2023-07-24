//! Expression nodes for Wolfram Language.

use crate::{
    ast::root_nodes::{Identifier, Span},
    lexer::token_type::WolframTokenType,
};

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
            Self::Literal { span, .. } | Self::List { span, .. } | Self::Call { span, .. } | Self::Part { span, .. } | Self::Blank { span, .. } | Self::Pattern { span, .. } | Self::Grouped { span, .. } | Self::Error { span } => span.clone(),
            Self::Binary(b) => b.span.clone(),
            Self::Prefix(u) | Self::Postfix(u) => u.span.clone(),
        }
    }

    /// Symbol / keyword name node.
    pub fn as_symbol(&self) -> Option<&Identifier> {
        match self {
            Self::Symbol(id) => Some(id),
            _ => None,
        }
    }

    /// Raw literal text and span.
    pub fn as_literal(&self) -> Option<(&str, &Span)> {
        match self {
            Self::Literal { value, span } => Some((value.as_str(), span)),
            _ => None,
        }
    }

    /// List elements.
    pub fn as_list(&self) -> Option<&[Expression]> {
        match self {
            Self::List { elements, .. } => Some(elements.as_slice()),
            _ => None,
        }
    }

    /// Call head and flattened arguments.
    pub fn as_call(&self) -> Option<(&Expression, &[Expression])> {
        match self {
            Self::Call { head, arguments, .. } => Some((head.as_ref(), arguments.as_slice())),
            _ => None,
        }
    }

    /// Part expression and indices.
    pub fn as_part(&self) -> Option<(&Expression, &[Expression])> {
        match self {
            Self::Part { expression, indices, .. } => Some((expression.as_ref(), indices.as_slice())),
            _ => None,
        }
    }

    /// Binary / infix node.
    pub fn as_binary(&self) -> Option<&BinaryExpr> {
        match self {
            Self::Binary(bin) => Some(bin.as_ref()),
            _ => None,
        }
    }

    /// Prefix unary node.
    pub fn as_prefix(&self) -> Option<&UnaryExpr> {
        match self {
            Self::Prefix(u) => Some(u.as_ref()),
            _ => None,
        }
    }

    /// Postfix unary node.
    pub fn as_postfix(&self) -> Option<&UnaryExpr> {
        match self {
            Self::Postfix(u) => Some(u.as_ref()),
            _ => None,
        }
    }

    /// Blank `_` / `__` / `___` with optional typed head.
    pub fn as_blank(&self) -> Option<(WolframTokenType, Option<&Expression>)> {
        match self {
            Self::Blank { kind, head, .. } => Some((*kind, head.as_deref())),
            _ => None,
        }
    }

    /// Named pattern `x_`.
    pub fn as_pattern(&self) -> Option<(&Expression, WolframTokenType)> {
        match self {
            Self::Pattern { name, blank, .. } => Some((name.as_ref(), *blank)),
            _ => None,
        }
    }

    /// Parenthesized inner expression.
    pub fn as_grouped(&self) -> Option<&Expression> {
        match self {
            Self::Grouped { expression, .. } => Some(expression.as_ref()),
            _ => None,
        }
    }

    /// Recovery / error node.
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    /// Assignment `lhs = rhs` / `lhs := rhs` (immediate vs deferred).
    pub fn as_assignment(&self) -> Option<AssignmentView<'_>> {
        let bin = self.as_binary()?;
        let timing = match bin.operator {
            WolframTokenType::Assign | WolframTokenType::Set => AssignmentTiming::Immediate,
            WolframTokenType::SetDelayed => AssignmentTiming::Deferred,
            _ => return None,
        };
        Some(AssignmentView { lhs: &bin.lhs, rhs: &bin.rhs, timing, operator: bin.operator })
    }

    /// Replacement rule `lhs -> rhs` / `lhs :> rhs`.
    pub fn as_rule(&self) -> Option<RuleView<'_>> {
        let bin = self.as_binary()?;
        let delayed = match bin.operator {
            WolframTokenType::Arrow | WolframTokenType::Rule => false,
            WolframTokenType::RuleDelayed | WolframTokenType::RuleDelayedOp => true,
            _ => return None,
        };
        Some(RuleView { lhs: &bin.lhs, rhs: &bin.rhs, delayed, operator: bin.operator })
    }

    /// Compound `lhs ; rhs` (binary semicolon chain).
    pub fn as_compound(&self) -> Option<(&Expression, &Expression)> {
        let bin = self.as_binary()?;
        if bin.operator != WolframTokenType::Semicolon {
            return None;
        }
        Some((&bin.lhs, &bin.rhs))
    }
}

/// Immediate (`=`) vs deferred (`:=`) assignment timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssignmentTiming {
    /// Evaluate RHS when defining.
    Immediate,
    /// Evaluate RHS when using.
    Deferred,
}

/// Typed view over an assignment binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignmentView<'a> {
    /// Left-hand side.
    pub lhs: &'a Expression,
    /// Right-hand side.
    pub rhs: &'a Expression,
    /// Evaluation timing.
    pub timing: AssignmentTiming,
    /// Surface operator token.
    pub operator: WolframTokenType,
}

/// Typed view over a rule binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleView<'a> {
    /// Left-hand side / pattern.
    pub lhs: &'a Expression,
    /// Right-hand side / replacement.
    pub rhs: &'a Expression,
    /// Whether the rule is delayed (`:>` / `RuleDelayed`).
    pub delayed: bool,
    /// Surface operator token.
    pub operator: WolframTokenType,
}
