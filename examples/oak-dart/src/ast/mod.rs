use core::range::Range;
use serde::{Deserialize, Serialize};

/// Identifier in the Dart language
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root for Dart language
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DartRoot {
    pub items: Vec<Item>,
}

/// Top-level items: classes, functions, variable declarations, etc.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Item {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

/// Class declaration in Dart
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Function declaration in Dart
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Variable declaration in Dart
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
