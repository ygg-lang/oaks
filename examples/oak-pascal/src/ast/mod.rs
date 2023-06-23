#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Pascal AST 根节点
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PascalRoot {
    pub items: Vec<PascalItem>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// Pascal 项目
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum PascalItem {
    Program {
        name: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
    Declaration {
        content: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
    Statement {
        content: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        range: Range<usize>,
    },
}
