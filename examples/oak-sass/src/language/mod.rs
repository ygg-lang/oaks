use oak_core::language::Language;

/// Sass 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SassLanguage {
    // Sass 语言特有的配置，目前为空
}

impl SassLanguage {
    /// 创建 Sass 语言实例
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SassLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for SassLanguage {
    type SyntaxKind = crate::kind::SassSyntaxKind;
}
