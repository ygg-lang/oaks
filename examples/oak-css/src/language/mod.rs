#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// CSS language implementation for the Oaks framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
