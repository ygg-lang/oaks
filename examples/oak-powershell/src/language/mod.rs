// No imports needed for now
use oak_core::{Language, LanguageCategory};

pub type TypedRoot = crate::ast::PowerShellRoot;

#[derive(Debug, Default)]
pub struct PowerShellLanguage;

impl Language for PowerShellLanguage {
    const NAME: &'static str = "powershell";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PowerShellSyntaxKind;
    type ElementType = crate::kind::PowerShellSyntaxKind;
    type TypedRoot = crate::ast::PowerShellRoot;
}
