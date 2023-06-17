use crate::kind::HaskellSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone)]
pub struct HaskellLanguage;

impl Language for HaskellLanguage {
    type SyntaxKind = HaskellSyntaxKind;
}
