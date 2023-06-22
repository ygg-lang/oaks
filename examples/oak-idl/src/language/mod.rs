use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default)]
pub struct IdlLanguage {}

impl Language for IdlLanguage {
    const NAME: &'static str = "idl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::IdlSyntaxKind;
    type ElementType = crate::kind::IdlSyntaxKind;
    type TypedRoot = crate::ast::IdlRoot;
}
