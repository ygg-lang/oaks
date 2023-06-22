use core::range::Range;
use serde::{Deserialize, Serialize};

/// Typst AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypstRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl TypstRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
