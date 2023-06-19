use crate::NimSyntaxKind;
use oak_core::Language;

#[derive(Debug)]
pub struct NimLanguage {
    pub allow_comment: bool,
}

impl Language for NimLanguage {
    type SyntaxKind = NimSyntaxKind;
    type TypedRoot = ();
}

impl Default for NimLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
