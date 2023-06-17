use crate::kind::SoliditySyntaxKind;
use oak_core::Language;

pub struct SolidityLanguage {}

impl Language for SolidityLanguage {
    type SyntaxKind = SoliditySyntaxKind;
}
