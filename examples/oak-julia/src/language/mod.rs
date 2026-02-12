#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Julia language implementation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JuliaLanguage {
    /// Whether to allow comments in the source code.
    pub allow_comment: bool,
}

impl JuliaLanguage {
    /// Creates a new instance of the Julia language configuration.
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
