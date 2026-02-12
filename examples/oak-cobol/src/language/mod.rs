use crate::{lexer::CobolTokenType, parser::CobolElementType};
use oak_core::{Language, LanguageCategory};

/// COBOL language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CobolLanguage {
    /// Whether strict mode is enabled
    pub strict_mode: bool,
}

impl CobolLanguage {
    /// Creates a new COBOL language configuration
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for CobolLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for CobolLanguage {
    const NAME: &'static str = "cobol";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CobolTokenType;
    type ElementType = CobolElementType;
    type TypedRoot = ();
}
