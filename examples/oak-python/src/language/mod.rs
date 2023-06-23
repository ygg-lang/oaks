#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Python language definition.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PythonLanguage {}

impl PythonLanguage {
    /// Creates a new Python language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PythonLanguage {
    const NAME: &'static str = "python";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PythonTokenType;
    type ElementType = crate::parser::element_type::PythonElementType;
    type TypedRoot = crate::ast::PythonRoot;
}
