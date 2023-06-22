use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Default)]
pub struct HaskellLanguage;

impl Language for HaskellLanguage {
    const NAME: &'static str = "haskell";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::HaskellSyntaxKind;
    type ElementType = crate::kind::HaskellSyntaxKind;
    type TypedRoot = crate::ast::HaskellRoot;
}
