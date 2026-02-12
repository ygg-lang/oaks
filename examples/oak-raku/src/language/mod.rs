use oak_core::{Language, LanguageCategory};

/// Raku language implementation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RakuLanguage {}

impl RakuLanguage {
    /// Creates a new `RakuLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RakuLanguage {
    const NAME: &'static str = "raku";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::RakuTokenType;
    type ElementType = crate::parser::element_type::RakuElementType;
    type TypedRoot = crate::ast::RakuRoot;
}
