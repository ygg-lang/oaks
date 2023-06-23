#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrologLanguage {}

impl PrologLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PrologLanguage {
    const NAME: &'static str = "prolog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PrologTokenType;
    type ElementType = crate::parser::element_type::PrologElementType;
    type TypedRoot = crate::ast::PrologRoot;
}
