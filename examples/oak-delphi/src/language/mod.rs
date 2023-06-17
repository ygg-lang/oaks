use crate::kind::DelphiSyntaxKind;
use oak_core::Language;

/// Delphi 语言配置
#[derive(Debug, Clone)]
pub struct DelphiLanguage {
    /// 是否启用严格语法检查
    pub strict_syntax: bool,
    /// 是否支持 Unicode 字符串
    pub unicode_strings: bool,
}

impl Default for DelphiLanguage {
    fn default() -> Self {
        Self { strict_syntax: false, unicode_strings: true }
    }
}

impl Language for DelphiLanguage {
    type SyntaxKind = DelphiSyntaxKind;
}
