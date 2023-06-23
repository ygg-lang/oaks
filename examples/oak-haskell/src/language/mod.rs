#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Default)]
pub struct HaskellLanguage {}

impl HaskellLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for HaskellLanguage {
    const NAME: &'static str = "haskell";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::HaskellTokenType;
    type ElementType = crate::parser::element_type::HaskellElementType;
    type TypedRoot = crate::ast::HaskellRoot;
}
