#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod value;
pub use self::{
    de::{deserialize, from_str},
    ser::{serialize, to_string},
    value::{TomlArray, TomlTable, TomlValue},
};
use crate::{ast::TomlRoot, lexer::token_type::TomlTokenKind, parser::element_type::TomlElementType};
use oak_core::{Language, LanguageCategory};

/// The TOML language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlLanguage {
    /// Whether to allow braces to be on new lines.
    ///
    /// Standard TOML requires braces to be on the same line as the key,
    /// but some extensions allow braces to be on new lines.
    pub multiline_inline_table: bool,
}

impl TomlLanguage {
    /// Create a new instance of the TOML language.
    pub fn new() -> Self {
        Self { multiline_inline_table: false }
    }

    /// Create a new instance of the TOML language with custom settings.
    pub fn with_multiline_inline_table(self, enable: bool) -> Self {
        Self { multiline_inline_table: enable, ..self }
    }
}

impl Language for TomlLanguage {
    const NAME: &'static str = "toml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = TomlTokenKind;
    type ElementType = TomlElementType;
    type TypedRoot = TomlRoot;
}
