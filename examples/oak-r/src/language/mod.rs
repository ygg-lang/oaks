#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Defines the R language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RLanguage {}

impl RLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RLanguage {
    const NAME: &'static str = "r";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::RTokenType;
    type ElementType = crate::parser::element_type::RElementType;
    type TypedRoot = crate::ast::RRoot;
}
