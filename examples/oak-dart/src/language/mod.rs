#![doc = include_str!("readme.md")]
use crate::ast::DartRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Language definition for Dart
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DartLanguage {}

impl DartLanguage {
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
