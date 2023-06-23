use crate::ast::Statement;
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// TypeScript AST 根节点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeScriptRoot {
    pub statements: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl TypeScriptRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { statements: vec![], span }
    }
}
