use super::{Block, Expr, GenericParam, Identifier, Param, Pattern, Span, Type};

/// A field in a class or struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// The field name.
    pub name: Identifier,
    /// The field type.
    pub ty: Type,
    /// Optional default value expression.
    pub default: Option<Expr>,
    /// Annotations applied to the field.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// An attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attribute {
    /// The attribute name.
    pub name: Identifier,
    /// The attribute arguments.
    pub args: Vec<Expr>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A string literal node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringLiteral {
    /// DSL prefix (e.g., `s`, `f`, `r`, `sql`).
    pub prefix: Option<Identifier>,
    /// Number of quotes (1, 2, 3, 4, ...).
    pub quote_count: u8,
    /// String segments.
    pub segments: Vec<StringSegment>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A string segment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringSegment {
    /// Text content.
    Text {
        /// The text content.
        content: String,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Interpolation expression.
    Interpolation {
        /// The interpolation expression.
        expr: Box<Expr>,
        /// Whether this is a Fluent variable (with the ߷ marker).
        is_fluent: bool,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}

/// A function definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The function name.
    pub name: Identifier,
    /// Generic parameters for the function.
    pub generics: Vec<GenericParam>,
    /// The function parameters.
    pub params: Vec<Param>,
    /// Optional return type annotation.
    pub return_type: Option<Type>,
    /// The optional function body.
    pub body: Option<Block>,
    /// Annotations applied to the function.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
    /// Whether this function is final (cannot be overridden).
    pub is_final: bool,
}

/// An enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumVariant {
    /// The variant name.
    pub name: Identifier,
    /// The variant fields.
    pub fields: Vec<Field>,
    /// Annotations applied to the variant.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
    /// Optional value expression for flags (e.g., `READ = 1` or `ALL = READ | WRITE`).
    pub value: Option<Expr>,
}

/// A variant case
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantCase {
    /// The pattern for this case.
    pub pattern: Pattern,
    /// The body expression for this case.
    pub body: Expr,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
