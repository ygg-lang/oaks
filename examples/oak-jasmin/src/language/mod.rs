#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Jasmin language configuration.
#[derive(Debug, Default, Copy, Clone)]
pub struct JasminLanguage {
    /// Whether to enable extended instructions (e.g., invokedynamic).
    pub extended: bool,
    /// Whether to allow comments.
    pub comments: bool,
}

impl JasminLanguage {
    /// Creates a new `JasminLanguage` configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard `JasminLanguage` configuration with all features enabled.
    pub fn standard() -> Self {
        Self { extended: true, comments: true }
    }

    /// Creates a minimal `JasminLanguage` configuration with all features disabled.
    pub fn minimal() -> Self {
        Self { extended: false, comments: false }
    }
}

impl Language for JasminLanguage {
    const NAME: &'static str = "jasmin";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JasminTokenType;
    type ElementType = crate::parser::element_type::JasminElementType;
    type TypedRoot = crate::ast::JasminRoot;
}
