#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone)]
pub struct XmlLanguage {}

impl XmlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for XmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for XmlLanguage {
    const NAME: &'static str = "xml";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::XmlTokenType;
    type ElementType = crate::parser::element_type::XmlElementType;
    type TypedRoot = crate::ast::XmlRoot;
}
