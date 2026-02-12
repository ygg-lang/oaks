//! Wolfram language definition.

use oak_core::{Language, LanguageCategory};

/// The Wolfram language definition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WolframLanguage {}

impl WolframLanguage {
    /// Creates a new `WolframLanguage`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WolframLanguage {
    const NAME: &'static str = "wolfram";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::WolframTokenType;
    type ElementType = crate::parser::element_type::WolframElementType;
    type TypedRoot = ();
}
