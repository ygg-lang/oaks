use core::range::Range;
use serde::{Deserialize, Serialize};

/// Sass AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SassRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
