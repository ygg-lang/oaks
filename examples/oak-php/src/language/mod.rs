#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The type of the root node for PHP AST.
pub type TypedRoot = crate::ast::PhpRoot;

/// PHP language implementation.
///
/// This struct implements the [`Language`] trait for the PHP language.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpLanguage {}

impl PhpLanguage {
    /// Creates a new `PhpLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PhpLanguage {
    const NAME: &'static str = "php";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PhpTokenType;
    type ElementType = crate::parser::element_type::PhpElementType;
    type TypedRoot = crate::ast::PhpRoot;
}
