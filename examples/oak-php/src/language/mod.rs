use crate::kind::PhpSyntaxKind;
use oak_core::Language;

pub struct PhpLanguage {}

impl Language for PhpLanguage {
    type SyntaxKind = PhpSyntaxKind;
}
