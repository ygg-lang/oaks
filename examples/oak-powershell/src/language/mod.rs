#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type TypedRoot = crate::ast::PowerShellRoot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellLanguage {}

impl PowerShellLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PowerShellLanguage {
    const NAME: &'static str = "powershell";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PowerShellTokenType;
    type ElementType = crate::parser::element_type::PowerShellElementType;
    type TypedRoot = crate::ast::PowerShellRoot;
}
