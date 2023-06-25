use crate::{ast::RegexRoot, lexer::RegexTokenType, parser::RegexElementType};
use oak_core::{Language, LanguageCategory};

/// Regex language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexLanguage;

impl RegexLanguage {
    /// Creates a new Regex language configuration.
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegexLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for RegexLanguage {
    const NAME: &'static str = "regex";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = RegexTokenType;
    type ElementType = RegexElementType;
    type TypedRoot = RegexRoot;
}
