#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// PureScript language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurescriptLanguage {
    /// Language configuration.
    pub config: (),
}

impl Default for PurescriptLanguage {
    fn default() -> Self {
        Self { config: () }
    }
}

impl Language for PurescriptLanguage {
    const NAME: &'static str = "purescript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PurescriptTokenType;
    type ElementType = crate::parser::element_type::PurescriptElementType;
    type TypedRoot = ();
}
