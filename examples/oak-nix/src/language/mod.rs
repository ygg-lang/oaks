#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Configuration for the Nix language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixLanguage {
    /// Whether comments are allowed in the source code.
    pub allow_comment: bool,
}

impl NixLanguage {
    /// Creates a new `NixLanguage` configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for NixLanguage {
    const NAME: &'static str = "nix";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::NixTokenType;
    type ElementType = crate::parser::element_type::NixElementType;
    type TypedRoot = crate::ast::NixRoot;
}

impl Default for NixLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
