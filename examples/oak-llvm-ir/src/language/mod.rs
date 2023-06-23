#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LLvmLanguage {}

impl Language for LLvmLanguage {
    const NAME: &'static str = "llvm-ir";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::LLvmTokenType;
    type ElementType = crate::parser::element_type::LLvmElementType;
    type TypedRoot = crate::ast::LLirRoot;
}
