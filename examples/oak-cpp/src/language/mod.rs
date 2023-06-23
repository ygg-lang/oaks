#![doc = include_str!("readme.md")]
use crate::lexer::CppLexer;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Oaks 框架的 C++ 语言实现。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CppLanguage {}

impl Language for CppLanguage {
    const NAME: &'static str = "cpp";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::CppTokenType;
    type ElementType = crate::parser::CppElementType;
    type TypedRoot = ();
}

impl CppLanguage {
    /// 创建一个新的 `CppLanguage` 实例。
    pub fn new() -> Self {
        Self {}
    }

    /// 使用该语言配置创建一个 C++ 词法分析器。
    pub fn lexer(&self) -> CppLexer<'_> {
        CppLexer::new(self)
    }
}

impl Default for CppLanguage {
    fn default() -> Self {
        Self {}
    }
}
