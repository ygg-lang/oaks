#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Configuration for the WebAssembly Text (WAT) language.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WatLanguage {}

impl WatLanguage {
    /// Creates a new instance of the WAT language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WatLanguage {
    const NAME: &'static str = "wat";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::lexer::token_type::WatTokenType;
    type ElementType = crate::parser::element_type::WatElementType;
    type TypedRoot = crate::ast::WatRoot;
}
