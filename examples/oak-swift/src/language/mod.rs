#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwiftLanguage {}

impl SwiftLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SwiftLanguage {
    const NAME: &'static str = "swift";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::SwiftTokenType;
    type ElementType = crate::parser::element_type::SwiftElementType;
    type TypedRoot = crate::ast::SwiftRoot;
}
