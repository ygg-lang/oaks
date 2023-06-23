#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// JASM language binding and configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JasmLanguage {
    /// Whether to enable extended instructions (e.g., invokedynamic, etc.).
    pub extended: bool,
    /// Whether to allow comments.
    pub comments: bool,
}

impl JasmLanguage {
    /// Creates a new JASM language configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard JASM language configuration.
    pub fn standard() -> Self {
        Self { extended: true, comments: true }
    }

    /// Creates a minimal JASM language configuration.
    pub fn minimal() -> Self {
        Self { extended: false, comments: false }
    }
}

impl Language for JasmLanguage {
    const NAME: &'static str = "jasm";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JasmTokenType;
    type ElementType = crate::parser::element_type::JasmElementType;
    type TypedRoot = crate::ast::JasmRoot;
}
