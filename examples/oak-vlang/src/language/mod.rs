use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// V 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VLangLanguage {}

impl VLangLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VLangLanguage {
    const NAME: &'static str = "vlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VLangSyntaxKind;
    type ElementType = crate::kind::VLangSyntaxKind;
    type TypedRoot = ();
}
