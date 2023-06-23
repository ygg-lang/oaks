#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// JSON language implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsonLanguage {
    /// Whether to allow trailing commas in objects and arrays
    pub trailing_comma: bool,
    /// Whether to allow bare keys (unquoted keys) in objects
    pub bare_keys: bool,
    /// Whether to allow single-quoted strings
    pub single_quotes: bool,
    /// Whether to allow comments (both line and block)
    pub comments: bool,
    /// Whether to allow hexadecimal numbers (e.g., 0xDEADBEEF)
    pub hex_numbers: bool,
    /// Whether to allow Infinity, -Infinity, and NaN
    pub infinity_and_nan: bool,
}

impl JsonLanguage {
    /// Creates a new JSON language instance with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard JSON language instance (no extensions).
    pub fn standard() -> Self {
        Self::default()
    }

    /// Creates a JSON5 language instance with all extensions enabled.
    pub fn json5() -> Self {
        Self { trailing_comma: true, bare_keys: true, single_quotes: true, comments: true, hex_numbers: true, infinity_and_nan: true }
    }

    /// Creates a relaxed JSON language instance with all extensions enabled.
    pub fn relaxed() -> Self {
        Self { trailing_comma: true, bare_keys: true, single_quotes: true, comments: true, hex_numbers: true, infinity_and_nan: true }
    }
}

impl Default for JsonLanguage {
    fn default() -> Self {
        Self { trailing_comma: false, bare_keys: false, single_quotes: false, comments: false, hex_numbers: false, infinity_and_nan: false }
    }
}

impl Language for JsonLanguage {
    const NAME: &'static str = "json";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::JsonTokenType;
    type ElementType = crate::parser::element_type::JsonElementType;
    type TypedRoot = crate::ast::JsonRoot;
}
