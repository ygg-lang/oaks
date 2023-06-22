//! Lua 语言定义
//!
//! 定义Lua 语言的核心结构体，实现了 oak-core Language trait

use oak_core::{Language, LanguageCategory};

/// Lua 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LuaLanguage;

impl Language for LuaLanguage {
    const NAME: &'static str = "lua";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::LuaSyntaxKind;
    type ElementType = crate::kind::LuaSyntaxKind;
    type TypedRoot = crate::ast::LuaRoot;
}

impl LuaLanguage {
    /// 创建新的 Lua 语言实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for LuaLanguage {
    fn default() -> Self {
        Self::new()
    }
}
