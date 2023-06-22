use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// VHDL 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VhdlLanguage {}

impl VhdlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for VhdlLanguage {
    const NAME: &'static str = "vhdl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VhdlSyntaxKind;
    type ElementType = crate::kind::VhdlSyntaxKind;
    type TypedRoot = ();
}
