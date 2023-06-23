#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Smalltalk 语言定义
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct SmalltalkLanguage {}

impl SmalltalkLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SmalltalkLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for SmalltalkLanguage {
    const NAME: &'static str = "smalltalk";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::SmalltalkTokenType;
    type ElementType = crate::parser::element_type::SmalltalkElementType;
    type TypedRoot = ();
}
