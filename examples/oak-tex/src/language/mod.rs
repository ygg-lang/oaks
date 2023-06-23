#![doc = include_str!("readme.md")]
use crate::ast::TexRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// TeX language definition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexLanguage {}

impl TexLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for TexLanguage {
    const NAME: &'static str = "tex";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = crate::lexer::token_type::TexTokenType;
    type ElementType = crate::parser::element_type::TexElementType;
    type TypedRoot = crate::ast::TexRoot;
}
