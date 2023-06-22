use core::range::Range;
use serde::{Deserialize, Serialize};

/// Tailwind 文档根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailwindRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl TailwindRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
