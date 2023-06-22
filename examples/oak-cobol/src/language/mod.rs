use crate::{lexer::CobolTokenType, parser::CobolElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// COBOL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct CobolLanguage {}

impl CobolLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CobolLanguage {
    const NAME: &'static str = "cobol";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CobolTokenType;
    type ElementType = CobolElementType;
    type TypedRoot = ();
}
