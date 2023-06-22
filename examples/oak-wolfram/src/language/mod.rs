use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WolframLanguage {}

impl WolframLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WolframLanguage {
    const NAME: &'static str = "wolfram";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::WolframSyntaxKind;
    type ElementType = crate::kind::WolframSyntaxKind;
    type TypedRoot = ();
}
