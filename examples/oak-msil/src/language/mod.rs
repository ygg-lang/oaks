use crate::kind::MsilSyntaxKind;
use oak_core::language::Language;

/// MSIL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsilLanguage {
    /// 是否允许扩展指令
    pub extended_instructions: bool,
    /// 是否允许调试信息
    pub debug_info: bool,
    /// 是否严格模式
    pub strict_mode: bool,
}

impl MsilLanguage {
    /// 创建新的 MSIL 语言实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建标准 MSIL 语言实例
    pub fn standard() -> Self {
        Self { extended_instructions: false, debug_info: false, strict_mode: true }
    }

    /// 创建扩展 MSIL 语言实例
    pub fn extended() -> Self {
        Self { extended_instructions: true, debug_info: true, strict_mode: false }
    }
}

impl Default for MsilLanguage {
    fn default() -> Self {
        Self { extended_instructions: false, debug_info: false, strict_mode: false }
    }
}

impl Language for MsilLanguage {
    type SyntaxKind = MsilSyntaxKind;
    type TypedRoot = ();
}
