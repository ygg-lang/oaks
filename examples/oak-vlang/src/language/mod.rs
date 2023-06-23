#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// V 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VLangLanguage {}

impl VLangLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VLangLanguage {
    const NAME: &'static str = "vlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VLangTokenType;
    type ElementType = crate::parser::element_type::VLangElementType;
    type TypedRoot = ();
}
