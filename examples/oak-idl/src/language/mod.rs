#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdlLanguage {}

impl IdlLanguage {
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
