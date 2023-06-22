use crate::{lexer::AsciiDocTokenType, parser::AsciiDocElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AsciiDocLanguage;

impl Language for AsciiDocLanguage {
    const NAME: &'static str = "ascii-doc";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = AsciiDocTokenType;
    type ElementType = AsciiDocElementType;
    type TypedRoot = ();
}

impl Default for AsciiDocLanguage {
    fn default() -> Self {
        Self {}
    }
}
