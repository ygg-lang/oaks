#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// CSS language implementation for the Oaks framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CssLanguage {}

impl CssLanguage {
    /// Creates a new `CssLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CssLanguage {
    const NAME: &'static str = "css";
    const CATEGORY: LanguageCategory = LanguageCategory::StyleSheet;
    type TokenType = crate::lexer::CssTokenType;
    type ElementType = crate::parser::element_type::CssElementType;
    type TypedRoot = ();
}
