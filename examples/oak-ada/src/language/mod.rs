#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]
use crate::{ast::AdaRoot, lexer::AdaTokenType, parser::AdaElementType};
use oak_core::{Language, LanguageCategory};

/// Ada language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdaLanguage {
    /// Whether to enable Ada 2022 features.
    pub allow_ada_2022: bool,
    /// Whether to enable strict mode.
    pub strict_mode: bool,
}

impl AdaLanguage {
    /// Creates a new Ada language configuration.
    pub fn new() -> Self {
        Self { allow_ada_2022: true, strict_mode: false }
    }
}

impl Default for AdaLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for AdaLanguage {
    const NAME: &'static str = "ada";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = AdaTokenType;
    type ElementType = AdaElementType;
    type TypedRoot = AdaRoot;
}
