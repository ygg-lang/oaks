#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct YamlLanguage {}

impl YamlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for YamlLanguage {
    const NAME: &'static str = "yaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::YamlTokenType;
    type ElementType = crate::parser::element_type::YamlElementType;
    type TypedRoot = ();
}
