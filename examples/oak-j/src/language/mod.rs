#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]
use crate::ast::JRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// J 语言配置和元数据
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
}

impl JLanguage {
    /// 鍒涘缓鏂扮殑 J 璇█閰嶇疆
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for JLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for JLanguage {
    const NAME: &'static str = "j";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JTokenType;
    type ElementType = crate::parser::element_type::JElementType;
    type TypedRoot = JRoot;
}
