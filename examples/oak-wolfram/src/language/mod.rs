use crate::kind::WolframSyntaxKind;
use oak_core::Language;

pub struct WolframLanguage {}

impl Language for WolframLanguage {
    type SyntaxKind = WolframSyntaxKind;
}
