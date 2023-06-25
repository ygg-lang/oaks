use oak_core::{Language, LanguageCategory};

/// Configuration for the Mermaid language.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MermaidLanguage {}

impl MermaidLanguage {
    /// Creates a new Mermaid language configuration.
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
