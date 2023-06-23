#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Julia language implementation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JuliaLanguage {
    pub allow_comment: bool,
}

impl JuliaLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for JuliaLanguage {
    const NAME: &'static str = "julia";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JuliaTokenType;
    type ElementType = crate::parser::element_type::JuliaElementType;
    type TypedRoot = crate::ast::JuliaRoot;
}

impl Default for JuliaLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
