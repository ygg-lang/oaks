use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Ini 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IniLanguage {}

impl IniLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for IniLanguage {
    const NAME: &'static str = "ini";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::IniSyntaxKind;
    type ElementType = crate::kind::IniSyntaxKind;
    type TypedRoot = crate::ast::IniRoot;
}
