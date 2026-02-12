use crate::{ast::SmalltalkRoot, lexer::SmalltalkTokenType, parser::SmalltalkElementType};
use oak_core::{Language, LanguageCategory};

/// Smalltalk language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmalltalkLanguage {
    /// Whether strict mode is enabled
    pub strict_mode: bool,
}

impl SmalltalkLanguage {
    /// Creates a new Smalltalk language configuration
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for SmalltalkLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for SmalltalkLanguage {
    const NAME: &'static str = "smalltalk";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = SmalltalkTokenType;
    type ElementType = SmalltalkElementType;
    type TypedRoot = SmalltalkRoot;
}
