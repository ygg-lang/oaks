#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WolframLanguage {}

impl WolframLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WolframLanguage {
    const NAME: &'static str = "wolfram";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::WolframTokenType;
    type ElementType = crate::parser::element_type::WolframElementType;
    type TypedRoot = ();
}
