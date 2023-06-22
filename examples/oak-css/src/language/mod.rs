use crate::{lexer::CssTokenType, parser::CssElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct CssLanguage {}

impl CssLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CssLanguage {
    const NAME: &'static str = "css";
    const CATEGORY: LanguageCategory = LanguageCategory::StyleSheet;
    type TokenType = CssTokenType;
    type ElementType = CssElementType;
    type TypedRoot = ();
}
