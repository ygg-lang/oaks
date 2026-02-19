#![doc = include_str!("readme.md")]
use core::range::Range;

/// Lean root node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeanRoot {
    /// The byte range of the root node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

impl LeanRoot {
    /// Creates a new Lean root node with the given byte range.
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}
