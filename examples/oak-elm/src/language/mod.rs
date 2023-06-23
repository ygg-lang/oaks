#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElmLanguage {}

impl ElmLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ElmLanguage {
    const NAME: &'static str = "elm";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ElmTokenType;
    type ElementType = crate::parser::element_type::ElmElementType;
    type TypedRoot = ();
}
