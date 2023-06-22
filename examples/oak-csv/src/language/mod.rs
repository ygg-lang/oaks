use crate::{lexer::CsvTokenType, parser::CsvElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct CsvLanguage {}

impl CsvLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CsvLanguage {
    const NAME: &'static str = "csv";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = CsvTokenType;
    type ElementType = CsvElementType;
    type TypedRoot = ();
}
