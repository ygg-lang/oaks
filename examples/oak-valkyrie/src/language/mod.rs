use crate::{ast::ValkyrieRoot, lexer::ValkyrieTokenType, parser::ValkyrieElementType};
use oak_core::{Language, LanguageCategory};

/// Valkyrie language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieLanguage {
    /// Whether to enable strict mode
    pub strict_mode: bool,
}

impl ValkyrieLanguage {
    /// Creates a new Valkyrie language configuration.
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = ValkyrieTokenType;
    type ElementType = ValkyrieElementType;
    type TypedRoot = ValkyrieRoot;
}
