#![doc = include_str!("readme.md")]
use core::range::Range;

/// Twig document root node
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl TwigRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
