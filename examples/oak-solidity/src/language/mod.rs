use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SolidityLanguage {}

impl SolidityLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SolidityLanguage {
    const NAME: &'static str = "solidity";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::SoliditySyntaxKind;
    type ElementType = crate::kind::SoliditySyntaxKind;
    type TypedRoot = ();
}
