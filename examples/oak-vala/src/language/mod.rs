use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Vala 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValaLanguage {}

impl ValaLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ValaLanguage {
    const NAME: &'static str = "vala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ValaSyntaxKind;
    type ElementType = crate::kind::ValaSyntaxKind;
    type TypedRoot = crate::ast::ValaRoot;
}
