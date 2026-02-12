#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Python language definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    type ElementType = crate::parser::PythonElementType;
    type TypedRoot = crate::ast::PythonRoot;
}
