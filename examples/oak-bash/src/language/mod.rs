use crate::{ast::BashRoot, lexer::BashTokenType, parser::BashElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct BashLanguage {}

impl BashLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for BashLanguage {
    const NAME: &'static str = "bash";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = BashTokenType;
    type ElementType = BashElementType;
    type TypedRoot = BashRoot;
}
