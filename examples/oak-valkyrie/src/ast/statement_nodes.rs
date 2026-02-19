use super::{Attribute, Expr, Pattern, Span, Type};

/// A statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// A let binding statement.
    Let {
        /// Whether the binding is mutable.
        is_mutable: bool,
        /// The pattern to bind to.
        pattern: Pattern,
        /// The expression being bound.
        expr: Expr,
        /// Optional type annotation.
        ty: Option<Type>,
        /// Annotations applied to the statement.
        annotations: Vec<Attribute>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An expression statement.
    ExprStmt {
        /// The expression.
        expr: Expr,
        /// Whether the statement ends with a semicolon.
        semi: bool,
        /// Annotations applied to the statement.
        annotations: Vec<Attribute>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}
