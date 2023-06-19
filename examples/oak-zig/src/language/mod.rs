use crate::kind::ZigSyntaxKind;
use oak_core::Language;

pub struct ZigLanguage {}

impl Language for ZigLanguage {
    type SyntaxKind = ZigSyntaxKind;
    type TypedRoot = ();
}
