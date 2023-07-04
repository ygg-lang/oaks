use super::{Identifier, NamePath, Span, TermExpression};

/// A match arm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchArm {
    /// The pattern to match against.
    pub pattern: Pattern,
    /// Optional guard expression.
    pub guard: Option<TermExpression>,
    /// The body expression of the arm.
    pub body: TermExpression,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A pattern for matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern {
    /// A wildcard pattern that matches anything.
    Wildcard(Box<WildcardPattern>),
    /// A variable pattern that binds the matched value.
    Variable(Box<VariablePattern>),
    /// A literal pattern.
    Literal(Box<LiteralPattern>),
    /// A type pattern for matching types.
    Type(Box<TypePattern>),
    /// A class pattern for destructuring.
    ///
    /// ```v
    /// let Point { x, y } = p           // shorthand syntax
    /// let Point { x: a, y: b } = p     // explicit binding
    /// let Point { x, y: new_y } = p    // mixed syntax
    /// ```
    Class(Box<ClassPattern>),
    /// An else pattern (catch-all).
    Else(Box<ElsePattern>),
}

/// A wildcard pattern that matches anything.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WildcardPattern {
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A variable pattern that binds the matched value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariablePattern {
    /// The variable name.
    pub name: Identifier,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A literal pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiteralPattern {
    /// The literal value as a string.
    pub value: String,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A type pattern for matching types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypePattern {
    /// The type name path.
    pub name: NamePath,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A class pattern for destructuring.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassPattern {
    /// The class name path.
    pub name: NamePath,
    /// The field patterns. None for shorthand syntax.
    pub fields: Vec<(Identifier, Option<Pattern>)>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// An else pattern (catch-all).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElsePattern {
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
