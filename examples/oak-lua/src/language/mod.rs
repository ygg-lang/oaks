#![doc = include_str!("readme.md")]
//! Lua language definition.
//!
//! Defines the core structure for the Lua language, implementing the oak-core Language trait.

use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Lua language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaLanguage {}

impl Language for LuaLanguage {
    const NAME: &'static str = "lua";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::LuaTokenType;
    type ElementType = crate::parser::element_type::LuaElementType;
    type TypedRoot = crate::ast::LuaRoot;
}

impl LuaLanguage {
    /// Creates a new Lua language instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LuaLanguage {
    fn default() -> Self {
        Self {}
    }
}
