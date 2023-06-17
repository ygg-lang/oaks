use crate::kind::DHallSyntaxKind;
use oak_core::Language;

/// Dhall 语言配置
#[derive(Debug, Clone)]
pub struct DHallLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许 Unicode 标识
    pub allow_unicode: bool,
}

impl Default for DHallLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_unicode: true }
    }
}

impl Language for DHallLanguage {
    type SyntaxKind = DHallSyntaxKind;
}
