#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default)]
pub struct PascalLanguage {}

impl PascalLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PascalLanguage {
    const NAME: &'static str = "pascal";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PascalTokenType;
    type ElementType = crate::parser::element_type::PascalElementType;
    type TypedRoot = crate::ast::PascalRoot;
}
