#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Configuration for the IDL language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdlLanguage {}

impl IdlLanguage {
    /// Creates a new instance of the IDL language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for IdlLanguage {
    const NAME: &'static str = "idl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::IdlTokenType;
    type ElementType = crate::parser::element_type::IdlElementType;
    type TypedRoot = crate::ast::IdlRoot;
}
