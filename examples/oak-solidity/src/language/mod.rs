use crate::{ast::SolidityRoot, lexer::SolidityTokenType, parser::SolidityElementType};
use oak_core::{Language, LanguageCategory};

/// Solidity language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SolidityLanguage {
    /// Whether strict mode is enabled
    pub strict_mode: bool,
}

impl SolidityLanguage {
    /// Creates a new Solidity language configuration
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
}

impl Default for SolidityLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for SolidityLanguage {
    const NAME: &'static str = "solidity";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = SolidityTokenType;
    type ElementType = SolidityElementType;
    type TypedRoot = SolidityRoot;
}
