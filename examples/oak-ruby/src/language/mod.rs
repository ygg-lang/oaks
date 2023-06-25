#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Ruby language implementation
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RubyLanguage {}

impl RubyLanguage {
    /// Creates a new `RubyLanguage` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RubyLanguage {
    const NAME: &'static str = "ruby";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::RubyTokenType;
    type ElementType = crate::parser::element_type::RubyElementType;
    type TypedRoot = crate::ast::RubyRoot;
}
