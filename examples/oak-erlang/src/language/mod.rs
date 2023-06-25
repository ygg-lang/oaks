#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Erlang language configuration.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErlangLanguage {}

impl ErlangLanguage {
    /// Creates a new `ErlangLanguage` configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ErlangLanguage {
    const NAME: &'static str = "erlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ErlangTokenType;
    type ElementType = crate::parser::element_type::ErlangElementType;
    type TypedRoot = crate::ast::ErlangRoot;
}
