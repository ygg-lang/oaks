use crate::syntax::JasmSyntaxKind;
use oak_core::language::Language;

/// JASM 语言绑定与配置
#[derive(Clone, Debug, Default)]
pub struct JasmLanguage {
    /// 是否启用扩展指令（如 invokedynamic 等）
    pub extended: bool,
    /// 是否允许注释
    pub comments: bool,
}

impl JasmLanguage {
    pub fn standard() -> Self {
        Self { extended: true, comments: true }
    }

    pub fn minimal() -> Self {
        Self { extended: false, comments: false }
    }
}

impl Language for JasmLanguage {
    type SyntaxKind = JasmSyntaxKind;
    type TypedRoot = (); // TODO: 添加 AST 根类型
}
