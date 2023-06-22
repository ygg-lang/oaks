use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct IdlLanguage {}

impl IdlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for IdlLanguage {
    const NAME: &'static str = "idl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::IdlSyntaxKind;
    type ElementType = crate::kind::IdlSyntaxKind;
    type TypedRoot = crate::ast::IdlRoot;
}
