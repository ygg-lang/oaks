use crate::{ast::DejavuRoot, lexer::DejavuTokenType, parser::DejavuElementType};
use oak_core::{Language, LanguageCategory};

/// Dejavu language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DejavuLanguage;

impl DejavuLanguage {
    /// Creates a new Dejavu language configuration.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DejavuLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for DejavuLanguage {
    const NAME: &'static str = "dejavu";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = DejavuTokenType;
    type ElementType = DejavuElementType;
    type TypedRoot = DejavuRoot;
}
