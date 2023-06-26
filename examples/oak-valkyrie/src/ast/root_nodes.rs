use super::Item;

/// Source code span
pub type Span = oak_core::Range<usize>;

/// Loop keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoopKind {
    /// Using `loop` keyword (preferred).
    #[default]
    Loop,
    /// Using `for` keyword (deprecated, use `loop` instead).
    For,
}

/// Structure keyword kind for class-like definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StructureKind {
    /// Using `class` keyword.
    #[default]
    Class,
    /// Using `struct` keyword (deprecated, use `class` instead).
    Struct,
    /// Using `structure` keyword.
    Structure,
    /// Using `widget` keyword.
    Widget,
    /// Using `trait` keyword.
    Trait,
}

/// Enums keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnumsKind {
    /// Using `enums` keyword (preferred).
    #[default]
    Enums,
    /// Using `enum` keyword (deprecated, use `unity` instead).
    Enum,
    /// Using `unity` keyword (preferred alternative).
    Unity,
}

/// An identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The identifier name as a string.
    pub name: String,
    /// The source code span where this identifier appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

impl Default for Identifier {
    fn default() -> Self {
        Self { name: String::new(), span: Span::default() }
    }
}

/// A name path (e.g., `std::collections::HashMap`)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamePath {
    /// The individual identifier parts of the path.
    pub parts: Vec<Identifier>,
    /// The source code span covering the entire name path.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// Valkyrie root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieRoot {
    /// The collection of top-level items in the Valkyrie module.
    pub items: Vec<Item>,
}
