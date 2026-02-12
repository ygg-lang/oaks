#![doc = include_str!("readme.md")]
use crate::language::NimLanguage;
use core::range::Range;
use oak_core::tree::RedNode;
use std::sync::Arc;

/// A Nim module.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NimModule {
    /// Items in the module.
    pub items: Vec<Item>,
}

/// A top-level item in a Nim source file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A statement.
    Statement(Statement),
    /// A declaration.
    Declaration(Arc<Declaration>),
}

/// Represents a statement in Nim.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// An expression statement.
    Expression(Expression),
    /// A `static:` block for compile-time execution.
    Static(Arc<Block>),
    /// A `when` statement for compile-time conditional.
    When {
        /// Branches of the when statement.
        branches: Vec<WhenBranch>,
        /// Optional else branch.
        else_branch: Option<Arc<Block>>,
    },
    // Add more variants as needed
}

/// Represents a branch in a `when` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhenBranch {
    /// Condition for the branch.
    pub condition: Expression,
    /// Body of the branch.
    pub body: Arc<Block>,
}

/// Represents a declaration in Nim.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Declaration {
    /// Name of the declaration.
    pub name: String,
    /// Span of the declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an expression in Nim.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// A literal value.
    Literal(String),
    /// An identifier.
    Identifier(String),
    // Add more variants as needed
}

/// Represents a block of code in Nim.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// Statements in the block.
    pub statements: Vec<Statement>,
    /// Span of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
