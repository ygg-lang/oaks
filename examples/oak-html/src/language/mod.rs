use crate::kind::HtmlSyntaxKind;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct HtmlLanguage {}

impl HtmlLanguage {
    pub fn new() -> Self {
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
