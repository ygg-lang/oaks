#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Less language implementation for the Oaks framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LessLanguage {}

impl LessLanguage {
    /// Creates a new `LessLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for LessLanguage {
    const NAME: &'static str = "less";
    const CATEGORY: LanguageCategory = LanguageCategory::StyleSheet;
    type TokenType = crate::lexer::LessTokenType;
    type ElementType = crate::parser::element_type::LessElementType;
    type TypedRoot = crate::ast::LessRoot;
}
