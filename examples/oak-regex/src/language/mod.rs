use crate::ast::RegexRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Configuration for the regular expression language.
///
/// This structure defines the language configuration for the regex parser,
/// including options such as whether to ignore whitespace characters.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegexLanguage {
    /// Whether to ignore whitespace characters
    pub ignore_whitespace: bool,
}

impl RegexLanguage {
    /// Creates a new RegexLanguage instance.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Default implementation for RegexLanguage.
///
/// Creates a RegexLanguage instance with default settings.
impl Default for RegexLanguage {
    fn default() -> Self {
        Self { ignore_whitespace: false }
    }
}

/// Implementation of the Language trait for RegexLanguage.
///
/// This connects the language configuration to the specific syntax kinds
/// and AST root type used for regex parsing.
impl Language for RegexLanguage {
    const NAME: &'static str = "regex";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::kind::RegexSyntaxKind;
    type ElementType = crate::kind::RegexSyntaxKind;
    type TypedRoot = RegexRoot;
}
