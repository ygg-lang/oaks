#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HlslLanguage {
    pub allow_comment: bool,
}

impl HlslLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for HlslLanguage {
    const NAME: &'static str = "hlsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::HlslTokenType;
    type ElementType = crate::parser::element_type::HlslElementType;
    type TypedRoot = crate::ast::HlslRoot;
}

impl Default for HlslLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
