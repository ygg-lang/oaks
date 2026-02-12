#![doc = include_str!("readme.md")]
use core::range::Range;

/// Represents an identifier in D source code.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The textual name of the identifier
    pub name: String,
    /// Source code span where this identifier appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root node representing the entire D source file.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DRoot {
    /// Collection of top-level items in the D file
    pub items: Vec<Item>,
}

/// Top-level items that can appear in a D source file.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Module {
    /// The name of the module.
    pub name: Identifier,
    /// The span of the module declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an import declaration in D.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Import {
    /// The path being imported.
    pub path: Vec<Identifier>,
    /// The span of the import declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a class definition in D.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    /// The name of the class.
    pub name: Identifier,
    /// The span of the class definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a struct definition in D.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Struct {
    /// The name of the struct.
    pub name: Identifier,
    /// The span of the struct definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a function definition in D.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    /// The name of the function.
    pub name: Identifier,
    /// The span of the function definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
