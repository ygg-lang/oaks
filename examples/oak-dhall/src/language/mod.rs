#![doc = include_str!("readme.md")]
use crate::ast::DHallRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DHallLanguage {
    /// Allow unicode identifiers
    pub unicode_identifiers: bool,
}

impl DHallLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DHallLanguage {
    fn default() -> Self {
        Self { unicode_identifiers: true }
    }
}

impl Language for DHallLanguage {
    const NAME: &'static str = "dhall";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DHallTokenType;
    type ElementType = crate::parser::element_type::DHallElementType;
    type TypedRoot = DHallRoot;
}
