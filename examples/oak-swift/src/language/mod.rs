use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwiftLanguage;

impl Language for SwiftLanguage {
    const NAME: &'static str = "swift";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SwiftSyntaxKind;
    type ElementType = crate::kind::SwiftSyntaxKind;
    type TypedRoot = ();
}
