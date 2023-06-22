use core::range::Range;
use serde::{Deserialize, Serialize};

/// Twig 文档根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwigRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl TwigRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
