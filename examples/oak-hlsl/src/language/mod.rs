use crate::kind::HlslSyntaxKind;
use oak_core::Language;

#[derive(Debug)]
pub struct HlslLanguage {
    pub allow_comment: bool,
}

impl Language for HlslLanguage {
    type SyntaxKind = HlslSyntaxKind;
    type TypedRoot = ();
}


impl Default for HlslLanguage {
    fn default() -> Self {
        Self {
            allow_comment: true,
        }
    }
}