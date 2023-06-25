#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// INI language definition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IniLanguage {}

impl IniLanguage {
    /// Creates a new `IniLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for IniLanguage {
    const NAME: &'static str = "ini";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::IniTokenType;
    type ElementType = crate::parser::element_type::IniElementType;
    type TypedRoot = crate::ast::IniRoot;
}
