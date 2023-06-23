#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Ini 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IniLanguage {}

impl IniLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for IniLanguage {
    const NAME: &'static str = "ini";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::IniTokenType;
    type ElementType = crate::parser::element_type::IniElementType;
    type TypedRoot = crate::ast::IniRoot;
}
