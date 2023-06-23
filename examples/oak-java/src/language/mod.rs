#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JavaLanguage {}

impl JavaLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for JavaLanguage {
    const NAME: &'static str = "java";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JavaTokenType;
    type ElementType = crate::parser::element_type::JavaElementType;
    type TypedRoot = crate::ast::JavaRoot;
}
