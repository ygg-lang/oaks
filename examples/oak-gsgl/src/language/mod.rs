//! GSGL 语言定义
//!
//! 定义GSGL 语言的核心结构体，实现了 oak-core Language trait
use crate::syntax::GsglSyntaxKind;
use oak_core::Language;

/// GSGL 语言定义
#[derive(Debug, Clone)]
pub struct GsglLanguage;

impl Language for GsglLanguage {
    type SyntaxKind = GsglSyntaxKind;
}

impl GsglLanguage {
    /// 创建新的 GSGL 语言实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for GsglLanguage {
    fn default() -> Self {
        Self::new()
    }
}
