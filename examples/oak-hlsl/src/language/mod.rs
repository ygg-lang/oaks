use crate::kind::HlslSyntaxKind;
use oak_core::Language;

pub struct HlslLanguage {}

impl Language for HlslLanguage {
    type SyntaxKind = HlslSyntaxKind;
}
