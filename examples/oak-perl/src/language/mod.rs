use crate::kind::PerlSyntaxKind;
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default, Clone, Copy)]
pub struct PerlLanguage;

impl Language for PerlLanguage {
    const NAME: &'static str = "perl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = PerlSyntaxKind;
    type ElementType = PerlSyntaxKind;
    type TypedRoot = crate::ast::PerlRoot;
}
