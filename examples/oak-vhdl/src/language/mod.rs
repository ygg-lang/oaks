use oak_core::{Language, LanguageCategory};

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
    const NAME: &'static str = "vhdl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VhdlSyntaxKind;
    type ElementType = crate::kind::VhdlSyntaxKind;
    type TypedRoot = ();
}
