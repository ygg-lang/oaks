use core::range::Range;
use serde::{Deserialize, Serialize};

/// Scala 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalaRoot {
    #[serde(with = "oak_core::serde_range")]
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
