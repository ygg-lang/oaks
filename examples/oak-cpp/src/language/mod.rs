use crate::kind::CppSyntaxKind;
use oak_core::Language;

/// C++ 语言实现
#[derive(Debug, Clone)]
pub struct CppLanguage;

impl Language for CppLanguage {
    type SyntaxKind = CppSyntaxKind;
}
