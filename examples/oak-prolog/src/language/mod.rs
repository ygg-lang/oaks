#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Prolog language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrologLanguage {}

impl PrologLanguage {
    /// Create a new `PrologLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PrologLanguage {
    const NAME: &'static str = "prolog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PrologTokenType;
    type ElementType = crate::parser::element_type::PrologElementType;
    type TypedRoot = crate::ast::PrologRoot;
}
