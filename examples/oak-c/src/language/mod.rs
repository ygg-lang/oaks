use crate::kind::CSyntaxKind;
use oak_core::Language;

/// C 语言实现
#[derive(Debug, Clone)]
pub struct CLanguage;

impl Language for CLanguage {
    type SyntaxKind = CSyntaxKind;
}
