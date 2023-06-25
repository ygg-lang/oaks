#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root node for Pascal AST.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct PascalRoot {
    /// List of items in the program.
    pub items: Vec<PascalItem>,
    /// Source range of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// An item in a Pascal program.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum PascalItem {
    /// A program declaration.
    Program {
        /// Name of the program.
        name: String,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
    /// A declaration (e.g., var, const, type).
    Declaration {
        /// Content of the declaration.
        content: String,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
    /// A statement.
    Statement {
        /// Content of the statement.
        content: String,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
}
