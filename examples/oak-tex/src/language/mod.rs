#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// TeX language definition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexLanguage {}

impl TexLanguage {
    /// Creates a new TeX language definition.
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
