#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RhombusLanguage {}

impl RhombusLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RhombusLanguage {
    const NAME: &'static str = "rhombus";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::RhombusTokenType;
    type ElementType = crate::parser::element_type::RhombusElementType;
    type TypedRoot = ();
}
