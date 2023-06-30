#![doc = include_str!("readme.md")]

/// JSON value representation.
pub mod value;
pub use value::JsonValue;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
pub use self::{de::deserialize, de::from_str, ser::serialize, ser::to_string};
use oak_core::{Language, LanguageCategory};

/// The JSON language definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl Default for JsonLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl JsonLanguage {
    /// Create a new instance of the JSON language with custom settings.
    pub fn new(trailing_comma: bool, bare_keys: bool, single_quotes: bool, comments: bool, hex_numbers: bool, infinity_and_nan: bool) -> Self {
        Self { trailing_comma, bare_keys, single_quotes, comments, hex_numbers, infinity_and_nan }
    }

    /// Create a JSON language instance with strict ANSI JSON settings.
    pub fn standard() -> Self {
        Self { trailing_comma: false, bare_keys: false, single_quotes: false, comments: false, hex_numbers: false, infinity_and_nan: false }
    }

    /// Create a JSON language instance with JSON5 settings.
    pub fn json5() -> Self {
        Self { trailing_comma: true, bare_keys: true, single_quotes: true, comments: true, hex_numbers: true, infinity_and_nan: true }
    }

    /// Create a JSON language instance with relaxed settings (alias for JSON5).
    pub fn relaxed() -> Self {
        Self::json5()
    }
}

impl Language for JsonLanguage {
    const NAME: &'static str = "json";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::JsonTokenType;
    type ElementType = crate::parser::element_type::JsonElementType;
    type TypedRoot = crate::ast::JsonRoot;
}
