#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// GSGL language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GsglLanguage {}

impl GsglLanguage {
    /// Creates a new `GsglLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for GsglLanguage {
    const NAME: &'static str = "gsgl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::GsglTokenType;
    type ElementType = crate::parser::element_type::GsglElementType;
    type TypedRoot = crate::ast::GsglRoot;
}
