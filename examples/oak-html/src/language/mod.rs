#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the HTML language configuration for the Oaks framework.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HtmlLanguage {}

impl HtmlLanguage {
    /// Creates a new `HtmlLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for HtmlLanguage {
    const NAME: &'static str = "html";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::HtmlTokenType;
    type ElementType = crate::parser::element_type::HtmlElementType;
    type TypedRoot = crate::ast::HtmlDocument;
}
