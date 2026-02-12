#![doc = include_str!("readme.md")]
//! F# AST definitions

use core::range::Range;

/// The root node of an F# program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FSharpRoot {
    /// Items in the compilation unit
    pub items: Vec<Item>,
}

/// Top-level items in an F# program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Namespace declaration
    Namespace(NamespaceDeclaration),
    /// Module declaration
    Module(ModuleDeclaration),
    /// Open directive (open)
    Open(OpenDirective),
    /// Binding (let)
    Binding(Binding),
}

/// Namespace declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclaration {
    /// Namespace name
    pub name: String,
    /// Members
    pub items: Vec<Item>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleDeclaration {
    /// Module name
    pub name: String,
    /// Whether it is a top-level module
    pub is_top_level: bool,
    /// Members
    pub items: Vec<Item>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Open directive (open)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenDirective {
    /// Import path
    pub path: String,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Binding (let)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Binding {
    /// Binding name
    pub name: String,
    /// Whether it is a recursive binding (rec)
    pub is_rec: bool,
    /// Parameter list
    pub parameters: Vec<String>,
    /// Bound expression
    pub expression: Expression,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// F# expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Literal or identifier
    Simple(String),
    /// If expression
    If {
        /// Condition expression
        condition: Box<Expression>,
        /// Then branch
        then_branch: Box<Expression>,
        /// Else branch
        else_branch: Option<Box<Expression>>,
    },
}
