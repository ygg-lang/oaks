#![doc = include_str!("readme.md")]
use crate::{ast::CmdRoot, lexer::token_type::CmdTokenType, parser::element_type::CmdElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Windows Command (CMD) language configuration and metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
