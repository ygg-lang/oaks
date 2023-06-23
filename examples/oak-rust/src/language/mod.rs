#![doc = include_str!("readme.md")]
use crate::ast::RustRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration and metadata for the Rust language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RustLanguage {
    /// Allow using `unsafe` blocks and functions
    pub allow_unsafe: bool,
    /// Allow using `async` functions and blocks
    pub allow_async: bool,
    /// Enable experimental features
    pub experimental_features: bool,
}

impl RustLanguage {
    /// Creates a new default Rust language configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RustLanguage {
    fn default() -> Self {
        Self { allow_unsafe: true, allow_async: true, experimental_features: false }
    }
}

impl Language for RustLanguage {
    const NAME: &'static str = "rust";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::RustTokenType;
    type ElementType = crate::parser::RustElementType;
    type TypedRoot = RustRoot;
}
