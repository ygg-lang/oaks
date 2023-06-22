#![doc = include_str!("readme.md")]

use core::range::Range;
use serde::{Deserialize, Serialize};

/// Vampire 根节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VampireRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub formulas: Vec<VampireFormula>,
}

/// Vampire 公式
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VampireFormula {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub role: String,
    pub formula: String, // 暂时用字符串表示，实际应为逻辑树
}

/// Vampire 包含指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VampireInclude {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub path: String,
    pub selection: Vec<String>,
}
