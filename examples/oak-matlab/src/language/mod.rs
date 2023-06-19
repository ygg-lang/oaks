use crate::kind::MatlabSyntaxKind;
use oak_core::Language;

pub struct MatlabLanguage {}

impl Default for MatlabLanguage {
    fn default() -> Self {
        MatlabLanguage {}
    }
}

impl Language for MatlabLanguage {
    type SyntaxKind = MatlabSyntaxKind;
    type TypedRoot = ();
}
