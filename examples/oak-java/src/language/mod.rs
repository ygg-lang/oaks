use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JavaLanguage;

impl Language for JavaLanguage {
    const NAME: &'static str = "java";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JavaSyntaxKind;
    type ElementType = crate::kind::JavaSyntaxKind;
    type TypedRoot = crate::ast::JavaRoot;
}
