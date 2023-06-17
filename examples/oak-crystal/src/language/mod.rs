use crate::kind::CrystalSyntaxKind;
use oak_core::Language;

/// Crystal 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrystalLanguage;

impl Language for CrystalLanguage {
    type SyntaxKind = CrystalSyntaxKind;
}
