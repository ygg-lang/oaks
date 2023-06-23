#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZigLanguage {}

impl ZigLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ZigLanguage {
    const NAME: &'static str = "zig";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ZigTokenType;
    type ElementType = crate::parser::element_type::ZigElementType;
    type TypedRoot = ();
}
