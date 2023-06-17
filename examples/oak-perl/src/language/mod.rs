use crate::kind::PerlSyntaxKind;
use oak_core::Language;

pub struct PerlLanguage {}

impl Language for PerlLanguage {
    type SyntaxKind = PerlSyntaxKind;
}
