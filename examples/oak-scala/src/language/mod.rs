#![doc = include_str!("readme.md")]
use crate::ast::ScalaRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Scala language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScalaLanguage {
    // Scala language-specific configuration, currently empty.
}

impl ScalaLanguage {
    /// Creates a Scala language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScalaLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for ScalaLanguage {
    const NAME: &'static str = "scala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ScalaTokenType;
    type ElementType = crate::parser::element_type::ScalaElementType;
    type TypedRoot = ScalaRoot;
}
