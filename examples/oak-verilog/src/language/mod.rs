use oak_core::Language;

/// Verilog 语言定义
#[derive(Debug, Clone)]
pub struct VerilogLanguage;

impl VerilogLanguage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VerilogLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for VerilogLanguage {
    type SyntaxKind = crate::kind::VerilogKind;
}