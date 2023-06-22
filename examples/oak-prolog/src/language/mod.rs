use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct PrologLanguage {}

impl PrologLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PrologLanguage {
    const NAME: &'static str = "prolog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PrologSyntaxKind;
    type ElementType = crate::kind::PrologSyntaxKind;
    type TypedRoot = crate::ast::PrologRoot;
}
