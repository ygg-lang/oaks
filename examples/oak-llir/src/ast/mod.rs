use core::range::Range;
use serde::{Deserialize, Serialize};

/// LLIR 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlirRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
