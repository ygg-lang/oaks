use oak_core::language::{Language, LanguageCategory};

use crate::{lexer::TokenType, parser::ElementType};

/// Racket language definition.
pub struct RacketLanguage;

impl Language for RacketLanguage {
    const NAME: &'static str = "racket";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = TokenType;
    type ElementType = ElementType;
    type TypedRoot = ();
}
