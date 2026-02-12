#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Perl language implementation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PerlLanguage {}

impl PerlLanguage {
    /// Creates a new `PerlLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PerlLanguage {
    const NAME: &'static str = "perl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PerlTokenType;
    type ElementType = crate::parser::element_type::PerlElementType;
    type TypedRoot = crate::ast::PerlRoot;
}
