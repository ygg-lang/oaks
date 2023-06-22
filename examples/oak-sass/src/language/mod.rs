use crate::ast::SassRoot;
use oak_core::language::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Sass 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SassLanguage {
    // Sass 语言特有的配置，目前为空
}

impl SassLanguage {
    /// 创建 Sass 语言实例
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SassLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for SassLanguage {
    const NAME: &'static str = "sass";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SassSyntaxKind;
    type ElementType = crate::kind::SassSyntaxKind;
    type TypedRoot = SassRoot;
}
