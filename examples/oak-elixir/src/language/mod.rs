#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Language definition for Elixir.
pub struct ElixirLanguage {}

impl ElixirLanguage {
    /// Creates a new ElixirLanguage instance.
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
