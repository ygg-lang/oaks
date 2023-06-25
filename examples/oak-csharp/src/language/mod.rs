#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// C# language implementation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CSharpLanguage {}

impl CSharpLanguage {
    /// Creates a new `CSharpLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CSharpLanguage {
    const NAME: &'static str = "C#";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::CSharpTokenType;
    type ElementType = crate::parser::element_type::CSharpElementType;
    type TypedRoot = crate::ast::CSharpRoot;
}
