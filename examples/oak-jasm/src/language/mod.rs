use oak_core::language::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// JASM 语言绑定与配置
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JasmLanguage {
    /// 是否启用扩展指令（如 invokedynamic 等）
    pub extended: bool,
    /// 是否允许注释
    pub comments: bool,
}

impl JasmLanguage {
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

impl Language for JasmLanguage {
    const NAME: &'static str = "jasm";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::syntax::JasmSyntaxKind;
    type ElementType = crate::syntax::JasmSyntaxKind;
    type TypedRoot = crate::ast::JasmRoot;
}
