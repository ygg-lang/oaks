#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]
use crate::ast::AplRoot;
use oak_core::{Language, LanguageCategory};

/// APL language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AplLanguage {
    /// Whether to enable APL extension features.
    pub allow_extensions: bool,
    /// Whether to enable strict mode.
    pub strict_mode: bool,
}

impl AplLanguage {
    /// Creates a new APL language configuration.
    pub fn new() -> Self {
        Self { allow_extensions: true, strict_mode: false }
    }
}

impl Default for AplLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for AplLanguage {
    const NAME: &'static str = "apl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::AplTokenType;
    type ElementType = crate::parser::element_type::AplElementType;
    type TypedRoot = AplRoot;
}
