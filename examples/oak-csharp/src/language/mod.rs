#![doc = include_str!("readme.md")]
use crate::{lexer::CSharpTokenType, parser::CSharpElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// C# 语言实现
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CSharpLanguage {}

impl CSharpLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CSharpLanguage {
    const NAME: &'static str = "C#";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CSharpTokenType;
    type ElementType = crate::parser::element_type::CSharpElementType;
    type TypedRoot = crate::ast::CSharpRoot;
}
