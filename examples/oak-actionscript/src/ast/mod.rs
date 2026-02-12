#![doc = include_str!("readme.md")]
/// ActionScript AST definitions
use core::range::Range;

/// Identifier
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// Identifier name
    pub name: String,
    /// Range in the source code
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Root node of the ActionScript AST
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ActionScriptRoot {
    /// All top-level items in the source file
    pub items: Vec<ActionScriptItem>,
}

/// ActionScript top-level items
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ActionScriptItem {
    /// Package definition
    Package(PackageDeclaration),
    /// Class definition
    Class(ClassDeclaration),
    /// Interface definition
    Interface(InterfaceDeclaration),
    /// Function definition
    Function(FunctionDeclaration),
    /// Variable declaration
    Variable(VariableDeclaration),
    /// Import statement
    Import(ImportDeclaration),
}

/// Package declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct PackageDeclaration {
    /// Package name
    pub name: Option<Identifier>,
    /// Items in the package body
    pub items: Vec<ActionScriptItem>,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Class declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    /// Class name
    pub name: Identifier,
    /// Modifiers (public, private, internal, etc.)
    pub modifiers: Vec<String>,
    /// Base class
    pub extends: Option<Identifier>,
    /// Implemented interfaces
    pub implements: Vec<Identifier>,
    /// Items in the class body
    pub items: Vec<ActionScriptItem>,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDeclaration {
    /// Interface name
    pub name: Identifier,
    /// Base interfaces
    pub extends: Vec<Identifier>,
    /// Items in the interface body
    pub items: Vec<ActionScriptItem>,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    /// Function name
    pub name: Identifier,
    /// Parameter list
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<Identifier>,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    /// Parameter name
    pub name: Identifier,
    /// Parameter type
    pub type_annotation: Option<Identifier>,
}

/// Variable declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    /// Variable name
    pub name: Identifier,
    /// Variable type
    pub type_annotation: Option<Identifier>,
    /// Whether it is a constant
    pub is_const: bool,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Import declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ImportDeclaration {
    /// Import path
    pub path: String,
    /// Span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl Default for ActionScriptRoot {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
