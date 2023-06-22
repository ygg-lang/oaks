use crate::{ast::TexRoot, kind::TexSyntaxKind};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// TeX 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TexLanguage {}

impl TexLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for TexLanguage {
    const NAME: &'static str = "tex";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = TexSyntaxKind;
    type ElementType = TexSyntaxKind;
    type TypedRoot = TexRoot;
}
