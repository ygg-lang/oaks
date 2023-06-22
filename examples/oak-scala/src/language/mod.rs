use crate::ast::ScalaRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Scala 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
        Self {}
    }
}

impl Language for ScalaLanguage {
    const NAME: &'static str = "scala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ScalaSyntaxKind;
    type ElementType = crate::kind::ScalaSyntaxKind;
    type TypedRoot = ScalaRoot;
}
