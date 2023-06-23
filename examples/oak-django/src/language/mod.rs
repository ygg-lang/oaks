#![doc = include_str!("readme.md")]
use crate::ast::DjangoRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Django 模板语言配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DjangoLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许自定义标签
    pub allow_custom_tags: bool,
}

impl DjangoLanguage {
    /// 创建新的 Django 语言实例
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DjangoLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_custom_tags: true }
    }
}

impl Language for DjangoLanguage {
    const NAME: &'static str = "django";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DjangoTokenType;
    type ElementType = crate::parser::element_type::DjangoElementType;
    type TypedRoot = DjangoRoot;
}
