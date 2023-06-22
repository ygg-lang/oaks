use oak_core::{Language, LanguageCategory};

#[derive(Default)]
pub struct WgslLanguage;

impl Language for WgslLanguage {
    const NAME: &'static str = "wgsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::WgslSyntaxKind;
    type ElementType = crate::kind::WgslSyntaxKind;
    type TypedRoot = ();
}
