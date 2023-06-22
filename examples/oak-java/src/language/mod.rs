use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct JavaLanguage {}

impl JavaLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for JavaLanguage {
    const NAME: &'static str = "java";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JavaSyntaxKind;
    type ElementType = crate::kind::JavaSyntaxKind;
    type TypedRoot = crate::ast::JavaRoot;
}
