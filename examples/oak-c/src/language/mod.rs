use crate::{ast::CRoot, lexer::CTokenType, parser::CElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// C 语言实现
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CLanguage {}

impl CLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CLanguage {
    const NAME: &'static str = "c";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CTokenType;
    type ElementType = CElementType;
    type TypedRoot = CRoot;
}

impl Default for CLanguage {
    fn default() -> Self {
        Self {}
    }
}
