#![doc = include_str!("readme.md")]
use crate::{ast::CrystalRoot, lexer::CrystalTokenType, parser::CrystalElementType};
use oak_core::{Language, LanguageCategory};

/// Crystal 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CrystalLanguage {}

impl CrystalLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CrystalLanguage {
    const NAME: &'static str = "Crystal";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CrystalTokenType;
    type ElementType = crate::parser::element_type::CrystalElementType;
    type TypedRoot = CrystalRoot;
}
