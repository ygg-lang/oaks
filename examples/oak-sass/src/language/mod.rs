#![doc = include_str!("readme.md")]
use crate::ast::SassRoot;
use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Implementation of the Sass language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SassLanguage {
    // Sass-specific configuration, currently empty
}

impl SassLanguage {
    /// Creates a new Sass language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SassLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for SassLanguage {
    const NAME: &'static str = "sass";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::SassTokenType;
    type ElementType = crate::parser::element_type::SassElementType;
    type TypedRoot = SassRoot;
}
