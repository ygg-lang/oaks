use core::range::Range;
use serde::{Deserialize, Serialize};

/// Pascal AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PascalRoot {
    pub items: Vec<PascalItem>,
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

/// Pascal 项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PascalItem {
    Program {
        name: String,
        #[serde(with = "oak_core::serde_range")]
        range: Range<usize>,
    },
    Declaration {
        content: String,
        #[serde(with = "oak_core::serde_range")]
        range: Range<usize>,
    },
    Statement {
        content: String,
        #[serde(with = "oak_core::serde_range")]
        range: Range<usize>,
    },
}
