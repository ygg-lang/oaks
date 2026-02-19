#![doc = include_str!("readme.md")]
use core::range::Range;

/// Identifier in the Dart language
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    /// The identifier name text
    pub name: String,
    /// The source span where this identifier appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root for Dart language
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct DartRoot {
    /// Top-level items in the Dart source file
    pub items: Vec<Item>,
}

/// Top-level items: classes, functions, variable declarations, etc.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    /// A class declaration
    Class(ClassDeclaration),
    /// A function declaration
    Function(FunctionDeclaration),
    /// A variable declaration
    Variable(VariableDeclaration),
}

/// Class declaration in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    /// The class name
    pub name: Identifier,
    /// The class body containing members
    pub body: Vec<Item>,
    /// The source span of the entire class declaration
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function declaration in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    /// The function name
    pub name: Identifier,
    /// The function parameters
    pub parameters: Vec<Parameter>,
    /// The function body
    pub body: Vec<Item>,
    /// The source span of the entire function declaration
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable declaration in Dart.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    /// The variable name.
    pub name: Identifier,
    /// Optional type annotation.
    pub type_annotation: Option<Identifier>,
    /// Optional initial value.
    pub value: Option<Expression>,
    /// The source span of the entire variable declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter in a Dart function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    /// The parameter name.
    pub name: Identifier,
    /// Optional type annotation.
    pub type_annotation: Option<Identifier>,
}

/// Basic expression in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// An identifier expression
    Identifier(Identifier),
    /// A literal value (string, number, boolean, etc.)
    Literal(String),
}
