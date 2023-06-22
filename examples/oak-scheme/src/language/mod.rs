use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SchemeLanguage {}

impl SchemeLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SchemeLanguage {
    const NAME: &'static str = "scheme";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SchemeSyntaxKind;
    type ElementType = crate::kind::SchemeSyntaxKind;
    type TypedRoot = ();
}
