use oak_core::language::{Language, LanguageCategory};

/// Scss 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScssLanguage {
    // Scss 语言特有的配置，目前为空
}

impl ScssLanguage {
    /// 创建 Scss 语言实例
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScssLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for ScssLanguage {
    const NAME: &'static str = "scss";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ScssSyntaxKind;
    type ElementType = crate::kind::ScssSyntaxKind;
    type TypedRoot = ();
}
