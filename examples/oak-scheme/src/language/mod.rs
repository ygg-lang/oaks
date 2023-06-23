#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SchemeLanguage {}

impl SchemeLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SchemeLanguage {
    const NAME: &'static str = "scheme";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::SchemeTokenType;
    type ElementType = crate::parser::element_type::SchemeElementType;
    type TypedRoot = ();
}
