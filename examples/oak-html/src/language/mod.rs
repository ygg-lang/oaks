use crate::kind::HtmlSyntaxKind;
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone)]
pub struct HtmlLanguage {}

impl HtmlLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HtmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for HtmlLanguage {
    const NAME: &'static str = "html";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = HtmlSyntaxKind;
    type ElementType = HtmlSyntaxKind;
    type TypedRoot = crate::ast::HtmlDocument;
}
