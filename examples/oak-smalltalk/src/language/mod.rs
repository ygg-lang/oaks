use oak_core::{Language, LanguageCategory};

/// Smalltalk 语言定义
#[derive(Debug, Clone)]
pub struct SmalltalkLanguage;

impl SmalltalkLanguage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SmalltalkLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for SmalltalkLanguage {
    const NAME: &'static str = "smalltalk";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SmalltalkSyntaxKind;
    type ElementType = crate::kind::SmalltalkSyntaxKind;
    type TypedRoot = ();
}
