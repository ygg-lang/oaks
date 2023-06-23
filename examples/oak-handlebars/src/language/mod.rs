#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HandlebarsLanguage {}

impl HandlebarsLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

unsafe impl Send for HandlebarsLanguage {}
unsafe impl Sync for HandlebarsLanguage {}

impl Language for HandlebarsLanguage {
    const NAME: &'static str = "handlebars";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::HandlebarsTokenType;
    type ElementType = crate::parser::element_type::HandlebarsElementType;
    type TypedRoot = crate::ast::HandlebarsRoot;
}
