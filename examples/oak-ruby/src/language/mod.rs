#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Ruby 语言实现
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RubyLanguage {}

impl RubyLanguage {
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
