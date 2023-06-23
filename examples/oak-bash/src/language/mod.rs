#![doc = include_str!("readme.md")]
use crate::ast::BashRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Bash language configuration and metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BashLanguage {}

impl BashLanguage {
    /// Creates a new Bash language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for BashLanguage {
    const NAME: &'static str = "bash";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::BashTokenType;
    type ElementType = crate::parser::element_type::BashElementType;
    type TypedRoot = BashRoot;
}
