#![doc = include_str!("readme.md")]
//! GSGL language definition.
//!
//! Defines the core structure for GSGL language, implementing oak-core's Language trait.
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// GSGL language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GsglLanguage {}

impl GsglLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for GsglLanguage {
    const NAME: &'static str = "gsgl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::GsglTokenType;
    type ElementType = crate::parser::element_type::GsglElementType;
    type TypedRoot = ();
}
