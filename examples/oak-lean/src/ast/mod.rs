use core::range::Range;
use serde::{Deserialize, Serialize};

/// Lean 根节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeanRoot {
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

impl LeanRoot {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}
