use crate::{ast::TypstRoot, kind::TypstSyntaxKind};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Typst 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypstLanguage {}

impl TypstLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for TypstLanguage {
    const NAME: &'static str = "typst";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = TypstSyntaxKind;
    type ElementType = TypstSyntaxKind;
    type TypedRoot = TypstRoot;
}
