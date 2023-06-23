#![doc = include_str!("readme.md")]
#![allow(unused)]

use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Javadoc language implementation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JavadocLanguage {}

impl JavadocLanguage {
    /// Creates a new Javadoc language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for JavadocLanguage {
    const NAME: &'static str = "javadoc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JavadocTokenType;
    type ElementType = crate::parser::element_type::JavadocElementType;
    type TypedRoot = crate::ast::JavadocRoot;
}
