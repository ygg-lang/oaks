use crate::ast::DHallRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DHallLanguage {
    /// Allow unicode identifiers
    pub unicode_identifiers: bool,
}

impl DHallLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DHallLanguage {
    fn default() -> Self {
        Self { unicode_identifiers: true }
    }
}

impl Language for DHallLanguage {
    const NAME: &'static str = "dhall";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::DHallSyntaxKind;
    type ElementType = crate::kind::DHallSyntaxKind;
    type TypedRoot = DHallRoot;
}
