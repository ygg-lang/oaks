use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

pub type TypedRoot = crate::ast::PowerShellRoot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct PowerShellLanguage {}

impl PowerShellLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for PowerShellLanguage {
    const NAME: &'static str = "powershell";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PowerShellSyntaxKind;
    type ElementType = crate::kind::PowerShellSyntaxKind;
    type TypedRoot = crate::ast::PowerShellRoot;
}
