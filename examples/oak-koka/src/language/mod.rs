#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};

/// Koka language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KokaLanguage {
    /// Whether to enable strict mode.
    pub strict_mode: bool,
    /// Whether to allow experimental features.
    pub experimental_features: bool,
}

impl KokaLanguage {
    /// Creates a new Koka language instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard Koka language instance.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Creates a Koka language instance with experimental features enabled.
    pub fn experimental() -> Self {
        Self { strict_mode: false, experimental_features: true }
    }

    /// Creates a Koka language instance with strict mode enabled.
    pub fn strict() -> Self {
        Self { strict_mode: true, experimental_features: false }
    }
}

impl Default for KokaLanguage {
    fn default() -> Self {
        Self { strict_mode: false, experimental_features: false }
    }
}

impl Language for KokaLanguage {
    const NAME: &'static str = "koka";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::KokaTokenType;
    type ElementType = crate::parser::element_type::KokaElementType;
    type TypedRoot = crate::ast::KokaRoot;
}
