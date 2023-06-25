//! Crystal language configuration.

use crate::ast::CrystalRoot;
use oak_core::{Language, LanguageCategory};

/// Crystal language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CrystalLanguage {}

impl CrystalLanguage {
    /// Creates a new `CrystalLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CrystalLanguage {
    const NAME: &'static str = "Crystal";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CrystalTokenType;
    type ElementType = crate::parser::element_type::CrystalElementType;
    type TypedRoot = CrystalRoot;
}
