use oak_core::{Language, LanguageCategory};

pub struct ZigLanguage {}

impl Language for ZigLanguage {
    const NAME: &'static str = "zig";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ZigSyntaxKind;
    type ElementType = crate::kind::ZigSyntaxKind;
    type TypedRoot = ();
}
