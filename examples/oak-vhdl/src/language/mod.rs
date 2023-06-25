#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// VHDL language definition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VhdlLanguage {}

impl VhdlLanguage {
    /// Creates a new `VhdlLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VhdlLanguage {
    const NAME: &'static str = "vhdl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VhdlTokenType;
    type ElementType = crate::parser::element_type::VhdlElementType;
    type TypedRoot = crate::ast::VhdlRoot;
}
