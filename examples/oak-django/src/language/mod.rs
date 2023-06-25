#![doc = include_str!("readme.md")]
use crate::ast::DjangoRoot;
use oak_core::{Language, LanguageCategory};

/// Django template language configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DjangoLanguage {
    /// Whether to enable strict mode
    pub strict_mode: bool,
    /// Whether to allow custom tags
    pub allow_custom_tags: bool,
    /// Variable tag start
    pub variable_start: String,
    /// Variable tag end
    pub variable_end: String,
    /// Tag start
    pub tag_start: String,
    /// Tag end
    pub tag_end: String,
    /// Comment start
    pub comment_start: String,
    /// Comment end
    pub comment_end: String,
}

impl DjangoLanguage {
    /// Creates a new Django language instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DjangoLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_custom_tags: true, variable_start: "{{".to_string(), variable_end: "}}".to_string(), tag_start: "{%".to_string(), tag_end: "%}".to_string(), comment_start: "{#".to_string(), comment_end: "#}".to_string() }
    }
}

impl Language for DjangoLanguage {
    const NAME: &'static str = "django";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DjangoTokenType;
    type ElementType = crate::parser::element_type::DjangoElementType;
    type TypedRoot = DjangoRoot;
}
