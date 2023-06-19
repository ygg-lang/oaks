use crate::kind::NixSyntaxKind;
use oak_core::Language;

#[derive(Debug)]
pub struct NixLanguage {
    pub allow_comment: bool,
}

impl Language for NixLanguage {
    type SyntaxKind = NixSyntaxKind;
    type TypedRoot = ();
}

impl Default for NixLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
