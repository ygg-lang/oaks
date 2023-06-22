use oak_core::{Language, LanguageCategory};

pub type TypedRoot = crate::ast::PhpRoot;

#[derive(Debug, Default)]
pub struct PhpLanguage;

impl Language for PhpLanguage {
    const NAME: &'static str = "php";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PhpSyntaxKind;
    type ElementType = crate::kind::PhpSyntaxKind;
    type TypedRoot = crate::ast::PhpRoot;
}
