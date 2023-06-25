#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Implementation of the Coq language for the OAK parsing framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoqLanguage {}

impl CoqLanguage {
    /// Creates a new CoqLanguage.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CoqLanguage {
    const NAME: &'static str = "coq";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CoqTokenType;
    type ElementType = crate::parser::element_type::CoqElementType;
    type TypedRoot = crate::ast::CoqRoot;
}
