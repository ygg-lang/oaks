#![doc = include_str!("readme.md")]
use crate::{lexer::token_type::NoteTokenType, parser::element_type::NoteElementType};
use oak_core::{Language, LanguageCategory};

#[derive(Debug)]
pub struct NotedownLanguage {
    pub xml_call: bool,
}

impl Language for NotedownLanguage {
    const NAME: &'static str = "notedown";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::NoteTokenType;
    type ElementType = crate::parser::element_type::NoteElementType;
    type TypedRoot = crate::ast::NoteDocument;
}

impl Default for NotedownLanguage {
    fn default() -> Self {
        Self { xml_call: false }
    }
}
