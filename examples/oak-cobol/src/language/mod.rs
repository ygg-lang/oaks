use crate::kind::CobolSyntaxKind;
use oak_core::Language;

/// COBOL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CobolLanguage;

impl Language for CobolLanguage {
    type SyntaxKind = CobolSyntaxKind;
}
