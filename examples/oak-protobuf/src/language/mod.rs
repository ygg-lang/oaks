#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ProtobufLanguage {}

impl ProtobufLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ProtobufLanguage {
    const NAME: &'static str = "protobuf";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ProtobufTokenType;
    type ElementType = crate::parser::element_type::ProtobufElementType;
    type TypedRoot = crate::ast::ProtobufRoot;
}
