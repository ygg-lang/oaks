use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Go 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct GoLanguage {}

impl GoLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for GoLanguage {
    const NAME: &'static str = "go";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::GoSyntaxKind;
    type ElementType = crate::kind::GoSyntaxKind;
    type TypedRoot = crate::ast::GoRoot;
}
