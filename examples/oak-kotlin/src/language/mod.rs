#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Kotlin language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KotlinLanguage {
    /// Whether to enable strict mode.
    pub strict_mode: bool,
    /// Whether to allow experimental features.
    pub experimental_features: bool,
}

impl KotlinLanguage {
    /// Creates a new Kotlin language instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard Kotlin language instance.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Creates a Kotlin language instance with experimental features enabled.
    pub fn experimental() -> Self {
        Self { strict_mode: false, experimental_features: true }
    }

    /// Creates a Kotlin language instance with strict mode enabled.
    pub fn strict() -> Self {
        Self { strict_mode: true, experimental_features: false }
    }
}

impl Default for KotlinLanguage {
    fn default() -> Self {
        Self { strict_mode: false, experimental_features: false }
    }
}

impl Language for KotlinLanguage {
    const NAME: &'static str = "kotlin";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::KotlinTokenType;
    type ElementType = crate::parser::element_type::KotlinElementType;
    type TypedRoot = crate::ast::KotlinRoot;
}
