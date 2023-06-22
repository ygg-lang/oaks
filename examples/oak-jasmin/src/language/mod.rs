use oak_core::{Language, LanguageCategory};

/// JASMIN 语言绑定与配置
#[derive(Debug, Default, Copy, Clone)]
pub struct JasminLanguage {
    /// 是否启用扩展指令（如 invokedynamic 等）
    pub extended: bool,
    /// 是否允许注释
    pub comments: bool,
}

impl JasminLanguage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn standard() -> Self {
        Self { extended: true, comments: true }
    }

    pub fn minimal() -> Self {
        Self { extended: false, comments: false }
    }
}

impl Language for JasminLanguage {
    const NAME: &'static str = "jasmin";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JasminSyntaxKind;
    type ElementType = crate::kind::JasminSyntaxKind;
    type TypedRoot = crate::ast::JasminRoot;
}
