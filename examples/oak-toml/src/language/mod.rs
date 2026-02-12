#![doc = include_str!("readme.md")]
use crate::{lexer::TomlLexer, parser::TomlParser};
use oak_core::{Language, LanguageCategory};

/// Date-time format specification for TOML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DateTimeFormat {
    /// RFC 3339 date-time format (standard TOML).
    Rfc3339,
    // Other possible date-time formats
}

/// TOML language configuration and definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlLanguage {
    /// Whether to allow multiline string literals.
    pub allow_multiline_strings: bool,
    /// Whether to allow hexadecimal numeric literals.
    pub allow_hex_numbers: bool,
    /// The date-time format to use for parsing.
    pub datetime_format: DateTimeFormat,
}

impl Language for TomlLanguage {
    const NAME: &'static str = "toml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::TomlTokenType;
    type ElementType = crate::parser::element_type::TomlElementType;
    type TypedRoot = crate::ast::TomlRoot;
}

impl Default for TomlLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl TomlLanguage {
    /// Creates a new `TomlLanguage` with default (standard) settings.
    pub fn new() -> Self {
        Self::standard()
    }

    /// Returns the standard TOML language configuration.
    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    /// Creates a lexer for this language configuration.
    pub fn lexer(&self) -> TomlLexer<'_> {
        TomlLexer::new(self)
    }

    /// Creates a parser for this language configuration.
    pub fn parser(&self) -> TomlParser<'_> {
        TomlParser::new(self)
    }
}
