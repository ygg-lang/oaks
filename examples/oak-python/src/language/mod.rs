use oak_core::{Language, LanguageCategory};

/// Python 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PythonLanguage;

impl Language for PythonLanguage {
    const NAME: &'static str = "python";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PythonSyntaxKind;
    type ElementType = crate::kind::PythonSyntaxKind;
    type TypedRoot = crate::ast::PythonRoot;
}
