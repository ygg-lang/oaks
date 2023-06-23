#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Scala 根节点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScalaRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ScalaRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}

impl Default for ScalaRoot {
    fn default() -> Self {
        Self { span: Range { start: 0, end: 0 } }
    }
}
