use oak_core::{Language, LanguageCategory};

#[derive(Default)]
pub struct WatLanguage {}

impl WatLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WatLanguage {
    const NAME: &'static str = "wat";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::kind::WatSyntaxKind;
    type ElementType = crate::kind::WatSyntaxKind;
    type TypedRoot = ();
}
