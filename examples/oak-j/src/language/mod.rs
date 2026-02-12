#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]
use crate::ast::JRoot;
use oak_core::{Language, LanguageCategory};

/// J language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JLanguage {
    /// Whether to enable strict mode.
    pub strict_mode: bool,
}

impl JLanguage {
    /// Creates a new J language configuration.
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for JLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for JLanguage {
    const NAME: &'static str = "j";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JTokenType;
    type ElementType = crate::parser::element_type::JElementType;
    type TypedRoot = JRoot;
}
