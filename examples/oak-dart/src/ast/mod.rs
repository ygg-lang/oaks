#![doc = include_str!("readme.md")]
use core::range::Range;

/// Identifier in the Dart language
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root for Dart language
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct DartRoot {
    pub items: Vec<Item>,
}

/// Top-level items: classes, functions, variable declarations, etc.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

/// Class declaration in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub name: Identifier,
    pub body: Vec<Item>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function declaration in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Item>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable declaration in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub type_annotation: Option<Identifier>,
    pub value: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter in a Dart function
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub type_annotation: Option<Identifier>,
}

/// Basic expression in Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(String),
}
