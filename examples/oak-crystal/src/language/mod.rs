use crate::kind::CrystalSyntaxKind;
use oak_core::Language;

/// Crystal 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrystalLanguage;

impl CrystalLanguage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CrystalLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for CrystalLanguage {
    type SyntaxKind = CrystalSyntaxKind;
    type TypedRoot = ();
}
