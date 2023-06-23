use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MermaidLanguage {}

impl MermaidLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for MermaidLanguage {
    const NAME: &'static str = "mermaid";
    const CATEGORY: LanguageCategory = LanguageCategory::Modeling;

    type TokenType = crate::lexer::token_type::MermaidTokenType;
    type ElementType = crate::parser::element_type::MermaidElementType;
    type TypedRoot = crate::ast::MermaidRoot;
}
