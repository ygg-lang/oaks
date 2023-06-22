use crate::kind::PerlSyntaxKind;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlLanguage {}

impl PerlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PerlLanguage {
    const NAME: &'static str = "perl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = PerlSyntaxKind;
    type ElementType = PerlSyntaxKind;
    type TypedRoot = crate::ast::PerlRoot;
}
