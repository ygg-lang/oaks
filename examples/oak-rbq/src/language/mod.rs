use crate::kind::RbqSyntaxKind;
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RbqLanguage {}

impl RbqLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RbqLanguage {
    const NAME: &'static str = "RBQ";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = RbqSyntaxKind;
    type ElementType = RbqSyntaxKind;
    type TypedRoot = crate::ast::RbqRoot;
}
