#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Matlab language definition.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatlabLanguage {}

impl MatlabLanguage {
    /// Creates a new `MatlabLanguage` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MatlabLanguage {
    fn default() -> Self {
        MatlabLanguage {}
    }
}

impl Language for MatlabLanguage {
    const NAME: &'static str = "matlab";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::MatlabTokenType;
    type ElementType = crate::parser::element_type::MatlabElementType;
    type TypedRoot = crate::ast::MatlabRoot;
}
