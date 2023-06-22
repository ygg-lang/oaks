use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemeLanguage;

impl Language for SchemeLanguage {
    const NAME: &'static str = "scheme";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SchemeSyntaxKind;
    type ElementType = crate::kind::SchemeSyntaxKind;
    type TypedRoot = ();
}
