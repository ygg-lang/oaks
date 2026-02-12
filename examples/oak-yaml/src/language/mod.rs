#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// The YAML language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlLanguage {}

impl YamlLanguage {
    /// Create a new instance of the YAML language.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for YamlLanguage {
    const NAME: &'static str = "yaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::YamlTokenType;
    type ElementType = crate::parser::element_type::YamlElementType;
    type TypedRoot = crate::ast::YamlRoot;
}
