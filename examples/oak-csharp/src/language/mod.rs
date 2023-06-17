use crate::CSharpSyntaxKind;
use oak_core::Language;

/// C# 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CSharpLanguage;

impl Language for CSharpLanguage {
    type SyntaxKind = CSharpSyntaxKind;
}
