#![doc = include_str!("readme.md")]

use oak_core::{Language, LanguageCategory};

/// VB.NET language definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VbNetLanguage;

impl VbNetLanguage {
    /// Creates a new VbNetLanguage
    pub fn new() -> Self {
        Self
    }
}

impl Language for VbNetLanguage {
    const NAME: &'static str = "VB.NET";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VbNetTokenType;
    type ElementType = crate::parser::element_type::VbNetElementType;
    type TypedRoot = crate::ast::VbNetRoot;
}
