#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Typed root for the PowerShell language.
pub type TypedRoot = crate::ast::PowerShellRoot;

/// The PowerShell language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellLanguage {}

impl PowerShellLanguage {
    /// Creates a new `PowerShellLanguage`.
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
