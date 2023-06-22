use crate::kind::CoqSyntaxKind;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Implementation of the Coq language for the OAK parsing framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct CoqLanguage {}

impl CoqLanguage {
    /// Creates a new CoqLanguage.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CoqLanguage {
    const NAME: &'static str = "coq";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CoqSyntaxKind;
    type ElementType = CoqSyntaxKind;
    type TypedRoot = crate::ast::CoqRoot;
}
