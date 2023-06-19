use crate::{ast::ScalaRoot, kind::ScalaSyntaxKind};
use oak_core::Language;

/// Scala 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalaLanguage {
    // Scala 语言特有的配置，目前为空
}

impl ScalaLanguage {
    /// 创建 Scala 语言实例
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScalaLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for ScalaLanguage {
    type SyntaxKind = ScalaSyntaxKind;
    type TypedRoot = ScalaRoot;
}
