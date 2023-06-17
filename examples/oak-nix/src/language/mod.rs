use crate::kind::NixSyntaxKind;
use oak_core::Language;

pub struct NixLanguage {}

impl Language for NixLanguage {
    type SyntaxKind = NixSyntaxKind;
}
