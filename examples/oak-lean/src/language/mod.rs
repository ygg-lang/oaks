//! Lean 语言定义
//!
//! 定义Lean 语言的核心结构体，实现了 oak-core Language trait
use crate::syntax::LeanSyntaxKind;
use oak_core::Language;

/// Lean 语言定义
#[derive(Debug, Clone)]
pub struct LeanLanguage;

impl Language for LeanLanguage {
    type SyntaxKind = LeanSyntaxKind;
}

impl LeanLanguage {
    /// 创建新的 Lean 语言实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for LeanLanguage {
    fn default() -> Self {
        Self::new()
    }
}
