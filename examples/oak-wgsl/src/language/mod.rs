use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WgslLanguage {}

impl WgslLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WgslLanguage {
    const NAME: &'static str = "wgsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::WgslSyntaxKind;
    type ElementType = crate::kind::WgslSyntaxKind;
    type TypedRoot = ();
}
