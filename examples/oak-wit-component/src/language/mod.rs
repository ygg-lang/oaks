use crate::kind::WitSyntaxKind;
use oak_core::Language;

pub struct WitLanguage {}

impl Language for WitLanguage {
    type SyntaxKind = WitSyntaxKind;
    type TypedRoot = ();
}
