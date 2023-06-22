use oak_core::language::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LLvmLanguage {}

impl Language for LLvmLanguage {
    const NAME: &'static str = "llir";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::LLvmSyntaxKind;
    type ElementType = crate::kind::LLvmSyntaxKind;
    type TypedRoot = crate::ast::LlirRoot;
}
