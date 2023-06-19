use crate::kind::PythonSyntaxKind;
use oak_core::Language;

/// Python 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PythonLanguage;

impl Language for PythonLanguage {
    type SyntaxKind = PythonSyntaxKind;
    type TypedRoot = crate::ast::SourceFile;
}
