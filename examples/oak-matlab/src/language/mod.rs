#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatlabLanguage {}

impl MatlabLanguage {
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
