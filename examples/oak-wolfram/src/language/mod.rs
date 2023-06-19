use crate::kind::WolframSyntaxKind;
use oak_core::Language;

pub struct WolframLanguage {}

impl WolframLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WolframLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for WolframLanguage {
    type SyntaxKind = WolframSyntaxKind;
    type TypedRoot = ();
}
