#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Groovy language configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    type TypedRoot = crate::ast::GroovyRoot;
}

impl GroovyLanguage {
    /// Creates a new Groovy language instance.
    pub fn new() -> Self {
        Self::default()
    }
}
