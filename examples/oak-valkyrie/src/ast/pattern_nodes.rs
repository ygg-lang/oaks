use super::{Expr, Identifier, NamePath, Span};

/// A match arm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchArm {
    /// The pattern to match against.
    pub pattern: Pattern,
    /// Optional guard expression.
    pub guard: Option<Expr>,
    /// The body expression of the arm.
    pub body: Expr,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A pattern for matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern {
    /// A wildcard pattern that matches anything.
    Wildcard {
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A variable pattern that binds the matched value.
    Variable {
        /// The variable name.
        name: Identifier,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A literal pattern.
    Literal {
        /// The literal value as a string.
        value: String,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A type pattern for matching types.
    Type {
        /// The type name path.
        name: NamePath,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A class pattern for destructuring.
    ///
    /// ```v
    /// let Point { x, y } = p           // shorthand syntax
    /// let Point { x: a, y: b } = p     // explicit binding
    /// let Point { x, y: new_y } = p    // mixed syntax
    /// ```
    Class {
        /// The class name path.
        name: NamePath,
        /// The field patterns. None for shorthand syntax.
        fields: Vec<(Identifier, Option<Pattern>)>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An else pattern (catch-all).
    Else {
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}
