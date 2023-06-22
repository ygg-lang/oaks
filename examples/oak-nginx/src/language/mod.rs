use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NginxLanguage {
    /// 是否允许扩展指令
    pub allow_extensions: bool,
    /// 是否严格模式
    pub strict_mode: bool,
}

impl Default for NginxLanguage {
    fn default() -> Self {
        Self { allow_extensions: false, strict_mode: false }
    }
}

impl Language for NginxLanguage {
    const NAME: &'static str = "nginx";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::NginxSyntaxKind;
    type ElementType = crate::kind::NginxSyntaxKind;
    type TypedRoot = crate::ast::NginxRoot;
}

impl NginxLanguage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn standard() -> Self {
        Self { allow_extensions: false, strict_mode: true }
    }

    pub fn extended() -> Self {
        Self { allow_extensions: true, strict_mode: false }
    }
}
