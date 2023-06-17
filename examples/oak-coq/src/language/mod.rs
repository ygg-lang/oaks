use crate::kind::CoqSyntaxKind;
use oak_core::Language;

/// Coq 语言实现
#[derive(Debug, Clone)]
pub struct CoqLanguage;

impl Language for CoqLanguage {
    type SyntaxKind = CoqSyntaxKind;
}
