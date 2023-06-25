use oak_core::{Language, LanguageCategory};

/// Implementation of the D2 language for the Oak framework.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct D2Language {}

impl D2Language {
    /// Creates a new `D2Language` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for D2Language {
    const NAME: &'static str = "d2";
    const CATEGORY: LanguageCategory = LanguageCategory::Modeling;

    type TokenType = crate::lexer::token_type::D2TokenType;
    type ElementType = crate::parser::element_type::D2ElementType;
    type TypedRoot = crate::ast::D2Root;
}
