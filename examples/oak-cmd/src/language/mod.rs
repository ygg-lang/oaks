#![doc = include_str!("readme.md")]
use crate::{ast::CmdRoot, lexer::token_type::CmdTokenType, parser::element_type::CmdElementType};
use oak_core::{Language, LanguageCategory};

/// Windows Command (CMD) language configuration and metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CmdLanguage {}

impl CmdLanguage {
    /// Creates a new Cmd language configuration.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CmdLanguage {
    const NAME: &'static str = "cmd";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CmdTokenType;
    type ElementType = CmdElementType;
    type TypedRoot = CmdRoot;
}
