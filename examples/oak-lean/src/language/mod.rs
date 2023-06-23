#![doc = include_str!("readme.md")]
//! Lean language definition.
//!
//! Defines the core structure for the Lean language, implementing the oak-core Language trait.
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Lean language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeanLanguage {}

impl Language for LeanLanguage {
    const NAME: &'static str = "lean";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::LeanTokenType;
    type ElementType = crate::parser::element_type::LeanElementType;
    type TypedRoot = crate::ast::LeanRoot;
}

impl LeanLanguage {
    /// Creates a new Lean language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LeanLanguage {
    fn default() -> Self {
        Self {}
    }
}
