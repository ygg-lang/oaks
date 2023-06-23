#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NimLanguage {
    pub allow_comment: bool,
}

impl Language for NimLanguage {
    const NAME: &'static str = "nim";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::NimTokenType;
    type ElementType = crate::parser::element_type::NimElementType;
    type TypedRoot = crate::builder::NimRoot;
}

impl NimLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for NimLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
