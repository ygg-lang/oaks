use core::range::Range;
use serde::{Deserialize, Serialize};

/// TypeScript AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl TypeScriptRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
