use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructurizrLanguage {}

impl StructurizrLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for StructurizrLanguage {
    const NAME: &'static str = "structurizr";
    const CATEGORY: LanguageCategory = LanguageCategory::Modeling;

    type TokenType = crate::lexer::token_type::StructurizrTokenType;
    type ElementType = crate::parser::element_type::StructurizrElementType;
    type TypedRoot = crate::ast::StructurizrRoot;
}
