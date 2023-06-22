use oak_core::{Language, LanguageCategory};

/// Ini 语言定义
#[derive(Debug, Default, Clone, Copy)]
pub struct IniLanguage;

impl Language for IniLanguage {
    const NAME: &'static str = "ini";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::IniSyntaxKind;
    type ElementType = crate::kind::IniSyntaxKind;
    type TypedRoot = crate::ast::IniRoot;
}
