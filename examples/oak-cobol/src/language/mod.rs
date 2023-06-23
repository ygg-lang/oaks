#![doc = include_str!("readme.md")]
use crate::{lexer::CobolTokenType, parser::CobolElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// COBOL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CobolLanguage {}

impl CobolLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CobolLanguage {
    const NAME: &'static str = "cobol";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CobolTokenType;
    type ElementType = crate::parser::element_type::CobolElementType;
    type TypedRoot = crate::ast::CobolRoot;
}
