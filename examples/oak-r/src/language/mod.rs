use oak_core::{Language, LanguageCategory};

/// R 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RLanguage;

impl Language for RLanguage {
    const NAME: &'static str = "r";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::RSyntaxKind;
    type ElementType = crate::kind::RSyntaxKind;
    type TypedRoot = ();
}

impl Default for RLanguage {
    fn default() -> Self {
        Self
    }
}
