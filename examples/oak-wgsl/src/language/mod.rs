use crate::kind::WgslSyntaxKind;
use oak_core::Language;

pub struct WgslLanguage;

impl Language for WgslLanguage {
    type SyntaxKind = WgslSyntaxKind;
}
