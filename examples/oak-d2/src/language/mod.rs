use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct D2Language {}

impl D2Language {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for D2Language {
    const NAME: &'static str = "d2";
    const CATEGORY: LanguageCategory = LanguageCategory::Modeling;

    type TokenType = crate::lexer::token_type::D2TokenType;
    type ElementType = crate::parser::element_type::D2ElementType;
    type TypedRoot = crate::ast::D2Root;
}
