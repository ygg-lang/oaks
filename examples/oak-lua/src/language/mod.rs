//! Lua 语言定义
//!
//! 定义Lua 语言的核心结构体，实现了 oak-core Language trait

use crate::kind::LuaSyntaxKind;
use oak_core::Language;

/// Lua 语言定义
#[derive(Debug, Clone)]
pub struct LuaLanguage;

impl Language for LuaLanguage {
    type SyntaxKind = LuaSyntaxKind;
    type TypedRoot = ();
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
