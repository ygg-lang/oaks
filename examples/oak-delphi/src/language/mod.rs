#![doc = include_str!("readme.md")]
use crate::ast::DelphiRoot;
use oak_core::{Language, LanguageCategory};

/// Language definition for Delphi programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelphiLanguage {
    /// Whether to enable strict syntax checking
    pub strict_syntax: bool,
    /// Whether to support Unicode strings
    pub unicode_strings: bool,
}

impl DelphiLanguage {
    /// Creates a new `DelphiLanguage` configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DelphiLanguage {
    fn default() -> Self {
        Self { strict_syntax: false, unicode_strings: true }
    }
}

impl Language for DelphiLanguage {
    const NAME: &'static str = "delphi";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DelphiTokenType;
    type ElementType = crate::parser::element_type::DelphiElementType;
    type TypedRoot = DelphiRoot;
}
