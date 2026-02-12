#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Configuration for the Nginx language support.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NginxLanguage {
    /// Whether to allow extended directives.
    pub allow_extensions: bool,
    /// Whether to enable strict mode.
    pub strict_mode: bool,
}

impl Default for NginxLanguage {
    fn default() -> Self {
        Self { allow_extensions: false, strict_mode: false }
    }
}

impl Language for NginxLanguage {
    const NAME: &'static str = "nginx";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::NginxTokenType;
    type ElementType = crate::parser::element_type::NginxElementType;
    type TypedRoot = crate::ast::NginxRoot;
}

impl NginxLanguage {
    /// Creates a new `NginxLanguage` with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `NginxLanguage` with standard settings (strict mode enabled).
    pub fn standard() -> Self {
        Self { allow_extensions: false, strict_mode: true }
    }

    /// Creates a new `NginxLanguage` with extended settings (extensions allowed).
    pub fn extended() -> Self {
        Self { allow_extensions: true, strict_mode: false }
    }
}
