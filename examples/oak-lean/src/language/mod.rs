#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Lean language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeanLanguage {}

impl Language for LeanLanguage {
    const NAME: &'static str = "lean";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::LeanTokenType;
    type ElementType = crate::parser::element_type::LeanElementType;
    type TypedRoot = crate::ast::LeanRoot;
}

impl LeanLanguage {
    /// Creates a new Lean language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LeanLanguage {
    fn default() -> Self {
        Self {}
    }
}
