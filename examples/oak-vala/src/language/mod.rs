#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Vala 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValaLanguage {}

impl ValaLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ValaLanguage {
    const NAME: &'static str = "vala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ValaTokenType;
    type ElementType = crate::parser::element_type::ValaElementType;
    type TypedRoot = crate::ast::ValaRoot;
}
