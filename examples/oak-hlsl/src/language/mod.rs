use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HlslLanguage {
    pub allow_comment: bool,
}

impl HlslLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for HlslLanguage {
    const NAME: &'static str = "hlsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::HlslSyntaxKind;
    type ElementType = crate::kind::HlslSyntaxKind;
    type TypedRoot = crate::ast::HlslRoot;
}

impl Default for HlslLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
