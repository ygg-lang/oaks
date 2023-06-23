#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElixirLanguage {}

impl ElixirLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ElixirLanguage {
    const NAME: &'static str = "elixir";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ElixirTokenType;
    type ElementType = crate::parser::element_type::ElixirElementType;
    type TypedRoot = crate::ast::ElixirRoot;
}
