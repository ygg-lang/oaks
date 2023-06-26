#![doc = include_str!("readme.md")]
use crate::lexer::StylusLexer;
use oak_core::{Language, LanguageCategory};

/// Date and time format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DateTimeFormat {
    #[default]
    /// RFC 3339 date-time format.
    Rfc3339,
    // Other possible date time formats
}

/// Stylus language definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StylusLanguage {
    /// Whether multiline strings are allowed.
    pub allow_multiline_strings: bool,
    /// Whether hexadecimal numbers are allowed.
    pub allow_hex_numbers: bool,
    /// The date-time format to use.
    pub datetime_format: DateTimeFormat,
}

impl Default for StylusLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl Language for StylusLanguage {
    const NAME: &'static str = "stylus";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::StylusTokenType;
    type ElementType = crate::parser::element_type::StylusElementType;
    type TypedRoot = ();
}

impl StylusLanguage {
    /// Create a new standard Stylus language configuration.
    pub fn new() -> Self {
        Self::standard()
    }

    /// Create a standard Stylus language configuration.
    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    /// Create a lexer for this language configuration.
    pub fn lexer(&self) -> StylusLexer<'_> {
        StylusLexer::new(self)
    }
}
