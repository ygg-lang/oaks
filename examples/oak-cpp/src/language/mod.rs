#![doc = include_str!("readme.md")]
use crate::{ast::CppRoot, lexer::CppLexer};
use oak_core::{Language, LanguageCategory};

/// C++ language implementation for the Oaks framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CppLanguage {}

impl Language for CppLanguage {
    const NAME: &'static str = "cpp";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::CppTokenType;
    type ElementType = crate::parser::CppElementType;
    type TypedRoot = CppRoot;
}

impl CppLanguage {
    /// Creates a new `CppLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Creates a C++ lexer using this language configuration.
    pub fn lexer(&self) -> CppLexer<'_> {
        CppLexer::new(self)
    }
}

impl Default for CppLanguage {
    fn default() -> Self {
        Self {}
    }
}
