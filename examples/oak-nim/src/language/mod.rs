use crate::NimSyntaxKind;
use oak_core::Language;

pub struct NimLanguage {}

impl Language for NimLanguage {
    type SyntaxKind = NimSyntaxKind;
}
