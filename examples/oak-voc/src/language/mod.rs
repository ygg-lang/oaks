use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Voc 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VocLanguage {}

impl VocLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VocLanguage {
    const NAME: &'static str = "voc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VocSyntaxKind;
    type ElementType = crate::kind::VocSyntaxKind;
    type TypedRoot = ();
}
