#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod value;
pub use self::value::Value;
pub use self::{de::deserialize, de::from_str, ser::serialize, ser::to_string};
use oak_core::{Language, LanguageCategory};

/// The TOML language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlLanguage {
    /// Whether to allow braces to be on new lines.
    /// 
    /// Standard TOML requires braces to be on the same line as the key,
    /// but some extensions allow braces to be on new lines.
    pub allow_braces_on_new_line: bool,
}

impl TomlLanguage {
    /// Create a new instance of the TOML language.
    pub fn new() -> Self {
        Self {
            allow_braces_on_new_line: false,
        }
    }

    /// Create a new instance of the TOML language with custom settings.
    pub fn allow_braces_on_new_line(self) -> Self {
        Self {
            allow_braces_on_new_line: true,
            ..self
        }
    }
}

impl Language for TomlLanguage {
    const NAME: &'static str = "toml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::TomlTokenKind;
    type ElementType = crate::parser::element_type::TomlElementType;
    type TypedRoot = crate::ast::TomlRoot;
}
