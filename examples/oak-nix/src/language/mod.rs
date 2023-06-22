use oak_core::{Language, LanguageCategory};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NixLanguage {
    pub allow_comment: bool,
}

impl NixLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for NixLanguage {
    const NAME: &'static str = "nix";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::NixSyntaxKind;
    type ElementType = crate::kind::NixSyntaxKind;
    type TypedRoot = ();
}

impl Default for NixLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
