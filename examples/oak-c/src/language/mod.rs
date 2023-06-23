#![doc = include_str!("readme.md")]
use crate::ast::CRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// C language implementation for the Oaks framework.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CLanguage {}

impl CLanguage {
    /// Creates a new `CLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CLanguage {
    const NAME: &'static str = "c";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CTokenType;
    type ElementType = crate::parser::element_type::CElementType;
    type TypedRoot = CRoot;
}

impl Default for CLanguage {
    fn default() -> Self {
        Self {}
    }
}
