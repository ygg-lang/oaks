use crate::kind::SoliditySyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Default)]
pub struct SolidityLanguage {}

impl SolidityLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for SolidityLanguage {
    type SyntaxKind = SoliditySyntaxKind;
    type TypedRoot = ();
}
