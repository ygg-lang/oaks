use crate::{lexer::CTokenType, parser::CElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// C 语言实现
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CLanguage {}

impl Language for CLanguage {
    const NAME: &'static str = "c";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CTokenType;
    type ElementType = CElementType;
    type TypedRoot = ();
}

impl Default for CLanguage {
    fn default() -> Self {
        Self {}
    }
}
