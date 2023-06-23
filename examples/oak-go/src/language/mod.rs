#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Go language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GoLanguage {}

impl GoLanguage {
    /// Creates a new Go language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for GoLanguage {
    const NAME: &'static str = "go";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::GoTokenType;
    type ElementType = crate::parser::element_type::GoElementType;
    type TypedRoot = crate::ast::GoRoot;
}
