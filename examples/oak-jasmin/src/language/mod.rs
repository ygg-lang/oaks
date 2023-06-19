use crate::kind::JasminSyntaxKind;
use oak_core::Language;

/// JASMIN 语言绑定与配置
#[derive(Debug, Default, Copy, Clone)]
pub struct JasminLanguage {
    /// 是否启用扩展指令（如 invokedynamic 等）
    pub extended: bool,
    /// 是否允许注释
    pub comments: bool,
}

impl JasminLanguage {
    pub fn standard() -> Self {
        Self { extended: true, comments: true }
    }

    pub fn minimal() -> Self {
        Self { extended: false, comments: false }
    }
}

impl Language for JasminLanguage {
    type SyntaxKind = JasminSyntaxKind;
    type TypedRoot = (); // TODO: 定义 TypedRoot
}
