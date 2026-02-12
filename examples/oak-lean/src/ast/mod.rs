#![doc = include_str!("readme.md")]
use core::range::Range;

/// Lean root node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeanRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

impl LeanRoot {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}
