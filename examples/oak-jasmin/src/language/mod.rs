use oak_core::Language;

/// JASMIN 语言绑定与配
#[derive(Clone, Debug)]
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
    type SyntaxKind = crate::kind::JasminSyntaxKind;
}
