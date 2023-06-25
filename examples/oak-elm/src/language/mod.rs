#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Implementation of the Elm language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElmLanguage {}

impl ElmLanguage {
    /// Creates a new Elm language implementation.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ElmLanguage {
    const NAME: &'static str = "elm";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ElmTokenType;
    type ElementType = crate::parser::element_type::ElmElementType;
    type TypedRoot = crate::ast::ElmRoot;
}
