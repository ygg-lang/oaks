use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default)]
pub struct PascalLanguage {}

impl Language for PascalLanguage {
    const NAME: &'static str = "pascal";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PascalSyntaxKind;
    type ElementType = crate::kind::PascalSyntaxKind;
    type TypedRoot = crate::ast::PascalRoot;
}
