use crate::kind::DartSyntaxKind;
use oak_core::Language;

pub struct DartLanguage {}

impl Language for DartLanguage {
    type SyntaxKind = DartSyntaxKind;
}
