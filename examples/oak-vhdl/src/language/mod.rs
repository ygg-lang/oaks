use oak_core::Language;

/// VHDL 语言定义
#[derive(Debug, Clone)]
pub struct VhdlLanguage;

impl VhdlLanguage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VhdlLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for VhdlLanguage {
    type SyntaxKind = crate::kind::VhdlSyntaxKind;
}
