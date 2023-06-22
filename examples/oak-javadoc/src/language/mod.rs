#![allow(unused)]

use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct JavadocLanguage {}

impl JavadocLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for JavadocLanguage {
    const NAME: &'static str = "javadoc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JavadocSyntaxKind;
    type ElementType = crate::kind::JavadocSyntaxKind;
    type TypedRoot = crate::ast::JavadocRoot;
}
