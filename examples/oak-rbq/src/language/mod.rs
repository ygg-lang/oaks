#![doc = include_str!("readme.md")]
use crate::{lexer::token_type::RbqTokenType, parser::element_type::RbqElementType};
use oak_core::{Language, LanguageCategory};

/// RBQ language definition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqLanguage {}

impl RbqLanguage {
    /// Creates a new instance of `RbqLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RbqLanguage {
    const NAME: &'static str = "RBQ";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = RbqTokenType;
    type ElementType = RbqElementType;
    type TypedRoot = crate::ast::RbqRoot;
}
