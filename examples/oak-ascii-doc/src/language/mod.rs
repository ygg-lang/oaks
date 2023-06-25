#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// AsciiDoc language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AsciiDocLanguage {}

impl AsciiDocLanguage {
    /// Creates a new `AsciiDocLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for AsciiDocLanguage {
    const NAME: &'static str = "ascii-doc";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::AsciiDocTokenType;
    type ElementType = crate::parser::element_type::AsciiDocElementType;
    type TypedRoot = crate::ast::AsciiDocRoot;
}
