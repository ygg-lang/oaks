#![doc = include_str!("readme.md")]
use crate::ast::DRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Language definition for D programming language
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DLanguage {
    /// Whether to enable D2 features
    pub d2_features: bool,
    /// Whether to allow inline assembly
    pub inline_asm: bool,
    /// Whether to enable contract programming
    pub contracts: bool,
}

impl DLanguage {
    /// Create a new D language instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a standard D language instance
    pub fn standard() -> Self {
        Self { d2_features: true, inline_asm: true, contracts: true }
    }

    /// Create a minimal D language instance
    pub fn minimal() -> Self {
        Self { d2_features: false, inline_asm: false, contracts: false }
    }
}

impl Default for DLanguage {
    fn default() -> Self {
        Self { d2_features: true, inline_asm: false, contracts: true }
    }
}

impl Language for DLanguage {
    const NAME: &'static str = "d";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DTokenType;
    type ElementType = crate::parser::element_type::DElementType;
    type TypedRoot = DRoot;
}
