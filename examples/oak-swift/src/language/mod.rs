use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SwiftLanguage {}

impl SwiftLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SwiftLanguage {
    const NAME: &'static str = "swift";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SwiftSyntaxKind;
    type ElementType = crate::kind::SwiftSyntaxKind;
    type TypedRoot = ();
}
