use oak_core::{Language, LanguageCategory};

/// Go 语言定义
#[derive(Debug, Clone, Copy, Default)]
pub struct GoLanguage {}

impl Language for GoLanguage {
    const NAME: &'static str = "go";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::GoSyntaxKind;
    type ElementType = crate::kind::GoSyntaxKind;
    type TypedRoot = crate::ast::GoRoot;
}
