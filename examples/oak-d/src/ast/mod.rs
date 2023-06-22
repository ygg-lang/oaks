use core::range::Range;
use serde::{Deserialize, Serialize};

/// Represents an identifier in D source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    /// The textual name of the identifier
    pub name: String,
    /// Source code span where this identifier appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root node representing the entire D source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DRoot {
    /// Collection of top-level items in the D file
    pub items: Vec<Item>,
}

/// Top-level items that can appear in a D source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    /// A module declaration
    Module(Module),
    /// An import declaration
    Import(Import),
    /// A class definition
    Class(Class),
    /// A struct definition
    Struct(Struct),
    /// A function definition
    Function(Function),
}

/// Represents a module declaration in D.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Module {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents an import declaration in D.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Import {
    pub path: Vec<Identifier>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a class definition in D.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a struct definition in D.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a function definition in D.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
