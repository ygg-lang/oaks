use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Python 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct PythonLanguage {}

impl PythonLanguage {
    /// 创建新的 Python 语言配置
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PythonLanguage {
    const NAME: &'static str = "python";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PythonSyntaxKind;
    type ElementType = crate::kind::PythonSyntaxKind;
    type TypedRoot = crate::ast::PythonRoot;
}
