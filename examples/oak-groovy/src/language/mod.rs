#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Groovy language configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroovyLanguage {}

impl Default for GroovyLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for GroovyLanguage {
    const NAME: &'static str = "groovy";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::GroovyTokenType;
    type ElementType = crate::parser::element_type::GroovyElementType;
    type TypedRoot = ();
}

impl GroovyLanguage {
    /// Creates a new Groovy language instance.
    pub fn new() -> Self {
        Self::default()
    }
}
