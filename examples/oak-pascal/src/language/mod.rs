use crate::kind::PascalSyntaxKind;
use oak_core::Language;

pub struct PascalLanguage {}

impl Language for PascalLanguage {
    type SyntaxKind = PascalSyntaxKind;
}
