#![doc = include_str!("readme.md")]
use crate::{ast::TypstRoot, lexer::token_type::TypstTokenType};
use oak_core::{Language, LanguageCategory};

/// Typst language definition
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypstLanguage {}

impl TypstLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for TypstLanguage {
    const NAME: &'static str = "typst";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = crate::lexer::token_type::TypstTokenType;
    type ElementType = crate::parser::element_type::TypstElementType;
    type TypedRoot = crate::ast::TypstRoot;
}
