#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RacketLanguage {}

impl RacketLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RacketLanguage {
    const NAME: &'static str = "racket";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::RacketTokenType;
    type ElementType = crate::parser::element_type::RacketElementType;
    type TypedRoot = ();
}
