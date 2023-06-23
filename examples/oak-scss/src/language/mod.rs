#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Scss language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScssLanguage {
    // Scss language-specific configuration, currently empty.
}

impl ScssLanguage {
    /// Creates a Scss language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScssLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for ScssLanguage {
    const NAME: &'static str = "scss";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ScssTokenType;
    type ElementType = crate::parser::element_type::ScssElementType;
    type TypedRoot = ();
}
