#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Handlebars language definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandlebarsLanguage {
    /// Variable tag start
    pub variable_start: String,
    /// Variable tag end
    pub variable_end: String,
    /// Unescaped variable tag start
    pub unescaped_start: String,
    /// Unescaped variable tag end
    pub unescaped_end: String,
}

impl HandlebarsLanguage {
    /// Creates a new `HandlebarsLanguage` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HandlebarsLanguage {
    fn default() -> Self {
        Self { variable_start: "{{".to_string(), variable_end: "}}".to_string(), unescaped_start: "{{{".to_string(), unescaped_end: "}}}".to_string() }
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
