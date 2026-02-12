use oak_core::{Language, LanguageCategory};

/// Configuration for the Structurizr language.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructurizrLanguage {}

impl StructurizrLanguage {
    /// Creates a new Structurizr language configuration.
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
