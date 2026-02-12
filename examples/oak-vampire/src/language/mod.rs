use crate::{ast::VampireRoot, lexer::VampireTokenType, parser::VampireElementType};
use oak_core::{Language, LanguageCategory};

/// Vampire language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VampireLanguage;

impl VampireLanguage {
    /// Creates a new Vampire language configuration.
    pub fn new() -> Self {
        Self
    }
}

impl Default for VampireLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for VampireLanguage {
    const NAME: &'static str = "vampire";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = VampireTokenType;
    type ElementType = VampireElementType;
    type TypedRoot = crate::ast::VampireRoot;
}
