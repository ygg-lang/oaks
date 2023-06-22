use crate::kind::CoqSyntaxKind;
use oak_core::{Language, LanguageCategory};

/// Implementation of the Coq language for the OAK parsing framework.
#[derive(Debug, Clone, Default)]
pub struct CoqLanguage;

impl Language for CoqLanguage {
    const NAME: &'static str = "coq";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CoqSyntaxKind;
    type ElementType = CoqSyntaxKind;
    type TypedRoot = crate::ast::CoqRoot;
}
