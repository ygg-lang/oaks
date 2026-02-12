#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// V language configuration and metadata.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VLangLanguage {}

impl VLangLanguage {
    /// Creates a new `VLangLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VLangLanguage {
    const NAME: &'static str = "vlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VLangTokenType;
    type ElementType = crate::parser::element_type::VLangElementType;
    type TypedRoot = crate::ast::VRoot;
}
