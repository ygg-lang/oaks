use oak_core::{Language, LanguageCategory};

/// Represents the PlantUML language configuration.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlantUmlLanguage {}

impl PlantUmlLanguage {
    /// Create a new `PlantUmlLanguage` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for PlantUmlLanguage {
    const NAME: &'static str = "plantuml";
    const CATEGORY: LanguageCategory = LanguageCategory::Modeling;

    type TokenType = crate::lexer::token_type::PlantUmlTokenType;
    type ElementType = crate::parser::element_type::PlantUmlElementType;
    type TypedRoot = crate::ast::PlantUmlRoot;
}
