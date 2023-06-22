use crate::ast::DRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Language definition for D programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DLanguage {
    /// Whether to enable D2 features
    pub d2_features: bool,
    /// Whether to allow inline assembly
    pub inline_asm: bool,
    /// Whether to enable contract programming
    pub contracts: bool,
}

impl DLanguage {
    /// Create a new D language instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a standard D language instance
    pub fn standard() -> Self {
        Self { d2_features: true, inline_asm: true, contracts: true }
    }

    /// Create a minimal D language instance
    pub fn minimal() -> Self {
        Self { d2_features: false, inline_asm: false, contracts: false }
    }
}

impl Default for DLanguage {
    fn default() -> Self {
        Self { d2_features: true, inline_asm: false, contracts: true }
    }
}

impl Language for DLanguage {
    const NAME: &'static str = "d";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::DSyntaxKind;
    type ElementType = crate::kind::DSyntaxKind;
    type TypedRoot = DRoot;
}
