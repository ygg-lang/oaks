use crate::kind::DSyntaxKind;
use oak_core::Language;

/// D 语言定义
#[derive(Debug, Clone)]
pub struct DLanguage {
    /// 是否启用 D2 特性
    pub d2_features: bool,
    /// 是否允许内联汇编
    pub inline_asm: bool,
    /// 是否启用契约编程
    pub contracts: bool,
}

impl DLanguage {
    /// 创建新的 D 语言实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建标准 D 语言实例
    pub fn standard() -> Self {
        Self { d2_features: true, inline_asm: true, contracts: true }
    }

    /// 创建简化的 D 语言实例
    pub fn minimal() -> Self {
        Self { d2_features: false, inline_asm: false, contracts: false }
    }
}

impl Default for DLanguage {
    fn default() -> Self {
        Self { d2_features: true, inline_asm: false, contracts: true }
    }
}

impl Language for DLanguage {
    type SyntaxKind = DSyntaxKind;
    type TypedRoot = ();
}
