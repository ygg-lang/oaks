use crate::{lexer::CsvTokenType, parser::CsvElementType};
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CsvLanguage;

impl Language for CsvLanguage {
    const NAME: &'static str = "csv";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = CsvTokenType;
    type ElementType = CsvElementType;
    type TypedRoot = ();
}
