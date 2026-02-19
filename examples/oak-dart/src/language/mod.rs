#![doc = include_str!("readme.md")]
use crate::ast::DartRoot;
use oak_core::{Language, LanguageCategory};

/// Language definition for Dart
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DartLanguage {}

impl DartLanguage {
    /// Creates a new DartLanguage instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for DartLanguage {
    const NAME: &'static str = "dart";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DartTokenType;
    type ElementType = crate::parser::element_type::DartElementType;
    type TypedRoot = DartRoot;
}
