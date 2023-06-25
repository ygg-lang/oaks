#![doc = include_str!("readme.md")]
use core::range::Range;
use std::sync::Arc;

/// Zig language abstract syntax tree root.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZigRoot {
    /// Items in the source file.
    pub items: Vec<Item>,
}

/// A top-level item in a Zig source file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A declaration.
    Declaration(Arc<Declaration>),
    /// A container field.
    ContainerField(Arc<ContainerField>),
    /// A comptime block at top level.
    Comptime(Arc<Block>),
}

/// A Zig declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Declaration {
    /// Name of the declaration.
    pub name: String,
    /// Whether it's a comptime declaration.
    pub is_comptime: bool,
    /// Span of the declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A Zig container field.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContainerField {
    /// Name of the field.
    pub name: String,
    /// Span of the field.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an expression in Zig.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// A literal value.
    Literal(String),
    /// An identifier.
    Identifier(String),
    /// A comptime expression.
    Comptime(Box<Expression>),
    /// A block expression.
    Block(Arc<Block>),
    // Add more variants as needed
}

/// Represents a block of code in Zig.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// Statements in the block.
    pub statements: Vec<Statement>,
    /// Span of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a statement in Zig.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// An expression statement.
    Expression(Expression),
    /// A comptime statement.
    Comptime(Arc<Block>),
    /// A declaration statement.
    Declaration(Arc<Declaration>),
    /// A block statement.
    Block(Arc<Block>),
}
