//! Lean 语言定义
//!
//! 定义Lean 语言的核心结构体，实现了 oak-core Language trait
use oak_core::{Language, LanguageCategory};

/// Lean 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeanLanguage;

impl Language for LeanLanguage {
    const NAME: &'static str = "lean";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::LeanSyntaxKind;
    type ElementType = crate::kind::LeanSyntaxKind;
    type TypedRoot = crate::ast::LeanRoot;
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
