use oak_core::{Language, LanguageCategory};

#[derive(Debug)]
pub struct HlslLanguage {
    pub allow_comment: bool,
}

impl Language for HlslLanguage {
    const NAME: &'static str = "hlsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::HlslSyntaxKind;
    type ElementType = crate::kind::HlslSyntaxKind;
    type TypedRoot = crate::ast::HlslRoot;
}

impl Default for HlslLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
