#![doc = include_str!("readme.md")]

/// YAML value representation.
pub mod value;
pub use value::YamlValue;

#[cfg(feature = "serde")]
pub(crate) mod de;
#[cfg(feature = "serde")]
pub(crate) mod ser;
#[cfg(feature = "serde")]
pub use self::{de::deserialize, de::from_str, ser::serialize, ser::to_string};
use oak_core::{Language, LanguageCategory};

/// The YAML language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlLanguage {}

impl YamlLanguage {
    /// Create a new instance of the YAML language.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for YamlLanguage {
    const NAME: &'static str = "yaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::lexer::token_type::YamlTokenType;
    type ElementType = crate::parser::element_type::YamlElementType;
    type TypedRoot = crate::ast::YamlRoot;
}
