#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SolidityLanguage {}

impl SolidityLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SolidityLanguage {
    const NAME: &'static str = "solidity";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::SolidityTokenType;
    type ElementType = crate::parser::element_type::SolidityElementType;
    type TypedRoot = ();
}
