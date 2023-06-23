#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NixLanguage {
    pub allow_comment: bool,
}

impl NixLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for NixLanguage {
    const NAME: &'static str = "nix";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::NixTokenType;
    type ElementType = crate::parser::element_type::NixElementType;
    type TypedRoot = ();
}

impl Default for NixLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
