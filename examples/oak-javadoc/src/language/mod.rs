#![allow(unused)]

use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JavadocLanguage;

impl Language for JavadocLanguage {
    const NAME: &'static str = "javadoc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JavadocSyntaxKind;
    type ElementType = crate::kind::JavadocSyntaxKind;
    type TypedRoot = crate::ast::JavadocRoot;
}
