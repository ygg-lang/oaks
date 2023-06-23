#![doc = include_str!("readme.md")]
use crate::{lexer::AsciiDocTokenType, parser::AsciiDocElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AsciiDocLanguage {}

impl AsciiDocLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for AsciiDocLanguage {
    const NAME: &'static str = "ascii-doc";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::AsciiDocTokenType;
    type ElementType = crate::parser::element_type::AsciiDocElementType;
    type TypedRoot = ();
}
