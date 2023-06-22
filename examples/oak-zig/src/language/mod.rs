use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ZigLanguage {}

impl ZigLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ZigLanguage {
    const NAME: &'static str = "zig";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ZigSyntaxKind;
    type ElementType = crate::kind::ZigSyntaxKind;
    type TypedRoot = ();
}
