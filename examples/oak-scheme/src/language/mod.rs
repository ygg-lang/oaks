#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SchemeLanguage {}

impl SchemeLanguage {
    /// Creates a new SchemeLanguage instance.
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
