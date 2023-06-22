use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

pub type TypedRoot = crate::ast::PhpRoot;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhpLanguage {}

impl PhpLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PhpLanguage {
    const NAME: &'static str = "php";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PhpSyntaxKind;
    type ElementType = crate::kind::PhpSyntaxKind;
    type TypedRoot = crate::ast::PhpRoot;
}
