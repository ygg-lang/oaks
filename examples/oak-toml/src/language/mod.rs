#![doc = include_str!("readme.md")]
use crate::{lexer::TomlLexer, parser::TomlParser};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Date-time format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DateTimeFormat {
    Rfc3339,
    // Other possible date-time formats
}

/// TOML language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TomlLanguage {
    pub allow_multiline_strings: bool,
    pub allow_hex_numbers: bool,
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
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    pub fn lexer(&self) -> TomlLexer<'_> {
        TomlLexer::new(self)
    }

    pub fn parser(&self) -> TomlParser<'_> {
        TomlParser::new(self)
    }
}
