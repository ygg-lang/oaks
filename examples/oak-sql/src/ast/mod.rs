#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// SQL 根节点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SqlRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    span: Range<usize>,
}
