use crate::kind::RustSyntaxKind;
use oak_core::Language;

#[derive(Debug, Default)]
pub struct RustLanguage {}

impl Language for RustLanguage {
    type SyntaxKind = RustSyntaxKind;
}
