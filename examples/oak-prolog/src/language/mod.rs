use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PrologLanguage;

impl Language for PrologLanguage {
    const NAME: &'static str = "prolog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PrologSyntaxKind;
    type ElementType = crate::kind::PrologSyntaxKind;
    type TypedRoot = crate::ast::PrologRoot;
}
