use crate::{lexer::CobolTokenType, parser::CobolElementType};
use oak_core::{Language, LanguageCategory};

/// COBOL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CobolLanguage;

impl Language for CobolLanguage {
    const NAME: &'static str = "cobol";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CobolTokenType;
    type ElementType = CobolElementType;
    type TypedRoot = ();
}
