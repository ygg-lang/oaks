//! Statement nodes for MATLAB.

use crate::ast::{expression_nodes::Expression, root_nodes::Span};

/// A MATLAB statement (owned).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Expression used as a statement.
    Expr(Expression),
    /// `if … end`.
    If {
        /// Condition.
        condition: Expression,
        /// Then body.
        then_body: Vec<Statement>,
        /// `elseif` arms `(condition, body)`.
        elseifs: Vec<(Expression, Vec<Statement>)>,
        /// Optional `else` body.
        else_body: Vec<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// `while … end`.
    While {
        /// Loop condition.
        condition: Expression,
        /// Loop body.
        body: Vec<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// `for … end`.
    For {
        /// Header expression (typically `i = 1:n`).
        header: Expression,
        /// Loop body.
        body: Vec<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// `try … catch … end`.
    Try {
        /// Protected body.
        body: Vec<Statement>,
        /// Optional catch identifier (`catch ME`).
        catch_name: Option<Expression>,
        /// Catch body.
        catch_body: Vec<Statement>,
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

impl Statement {
    /// Source span.
    pub fn span(&self) -> Span {
        match self {
            Self::Expr(e) => e.span(),
            Self::If { span, .. }
            | Self::While { span, .. }
            | Self::For { span, .. }
            | Self::Try { span, .. }
            | Self::Error { span } => span.clone(),
        }
    }
}
