use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MatlabLanguage {}

impl MatlabLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MatlabLanguage {
    fn default() -> Self {
        MatlabLanguage {}
    }
}

impl Language for MatlabLanguage {
    const NAME: &'static str = "matlab";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::MatlabSyntaxKind;
    type ElementType = crate::kind::MatlabSyntaxKind;
    type TypedRoot = crate::ast::MatlabRoot;
}
