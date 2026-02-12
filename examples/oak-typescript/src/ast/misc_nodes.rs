use crate::ast::Expression;
use core::range::Range;

/// Represents a TypeScript decorator.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decorator {
    /// The expression being used as a decorator.
    pub expression: Expression,
    /// Source span of the decorator.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a member of a TypeScript enum.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumMember {
    /// Name of the enum member.
    pub name: String,
    /// Initializer expression of the enum member, if any.
    pub initializer: Option<Expression>,
    /// Source span of the enum member.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
