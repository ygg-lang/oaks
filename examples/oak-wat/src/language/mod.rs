use crate::kind::WatSyntaxKind;
use oak_core::Language;

pub struct WatLanguage;

impl Language for WatLanguage {
    type SyntaxKind = WatSyntaxKind;
    type TypedRoot = ();
}
