use crate::ast::DelphiRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Language definition for Delphi programming language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelphiLanguage {
    /// Whether to enable strict syntax checking
    pub strict_syntax: bool,
    /// Whether to support Unicode strings
    pub unicode_strings: bool,
}

impl Default for DelphiLanguage {
    fn default() -> Self {
        Self { strict_syntax: false, unicode_strings: true }
    }
}

impl Language for DelphiLanguage {
    const NAME: &'static str = "delphi";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::DelphiSyntaxKind;
    type ElementType = crate::kind::DelphiSyntaxKind;
    type TypedRoot = DelphiRoot;
}
