use crate::ast::DartRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Language definition for Dart
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DartLanguage {}

impl DartLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for DartLanguage {
    const NAME: &'static str = "dart";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::DartSyntaxKind;
    type ElementType = crate::kind::DartSyntaxKind;
    type TypedRoot = DartRoot;
}
