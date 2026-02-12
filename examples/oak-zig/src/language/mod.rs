#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Zig language configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZigLanguage {}

impl ZigLanguage {
    /// Creates a new Zig language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ZigLanguage {
    const NAME: &'static str = "zig";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ZigTokenType;
    type ElementType = crate::parser::element_type::ZigElementType;
    type TypedRoot = crate::ast::ZigRoot;
}
