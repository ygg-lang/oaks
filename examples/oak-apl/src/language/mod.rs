#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]
use crate::{ast::AplRoot, lexer::AplTokenType, parser::AplElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// APL 语言配置和元数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AplLanguage {
    /// 是否启用 APL 扩展特性
    pub allow_extensions: bool,
    /// 是否启用严格模式
    pub strict_mode: bool,
}

impl AplLanguage {
    /// 创建新的 APL 语言配置
    pub fn new() -> Self {
        Self { allow_extensions: true, strict_mode: false }
    }
}

impl Default for AplLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for AplLanguage {
    const NAME: &'static str = "apl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::AplTokenType;
    type ElementType = crate::parser::element_type::AplElementType;
    type TypedRoot = AplRoot;
}
