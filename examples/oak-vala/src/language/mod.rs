#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Vala language definition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaLanguage {}

impl ValaLanguage {
    /// Creates a new `ValaLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ValaLanguage {
    const NAME: &'static str = "vala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ValaTokenType;
    type ElementType = crate::parser::element_type::ValaElementType;
    type TypedRoot = crate::ast::ValaRoot;
}
