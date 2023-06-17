use oak_core::Language;

pub struct BashLanguage;

impl Language for BashLanguage {
    type SyntaxKind = crate::kind::BashSyntaxKind;
}
