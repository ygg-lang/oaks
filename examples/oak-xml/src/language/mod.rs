//! XML language definition.

use oak_core::{Language, LanguageCategory};

#[cfg(feature = "serde")]
/// Serialization and deserialization utilities for XML.
pub mod serde;
#[cfg(feature = "serde")]
pub use serde::{from_value, to_value};

/// XML language.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct XmlLanguage {}

impl XmlLanguage {
    /// Creates a new `XmlLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for XmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for XmlLanguage {
    const NAME: &'static str = "xml";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::XmlTokenType;
    type ElementType = crate::parser::element_type::XmlElementType;
    type TypedRoot = crate::ast::XmlRoot;
}
