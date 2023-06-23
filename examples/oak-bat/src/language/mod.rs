#![doc = include_str!("readme.md")]
use crate::{ast::BatRoot, lexer::token_type::BatTokenType, parser::element_type::BatElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Windows Batch (BAT) language configuration and metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BatLanguage {}

impl BatLanguage {
    /// Creates a new Bat language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for BatLanguage {
    const NAME: &'static str = "bat";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = BatTokenType;
    type ElementType = BatElementType;
    type TypedRoot = BatRoot;
}
