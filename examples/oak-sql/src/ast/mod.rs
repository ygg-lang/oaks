use core::range::Range;
use serde::{Deserialize, Serialize};

/// SQL 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlRoot {
    #[serde(with = "oak_core::serde_range")]
    span: Range<usize>,
}
