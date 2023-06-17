use crate::kind::AsciiDocSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiDocLanguage;

impl Language for AsciiDocLanguage {
    type SyntaxKind = AsciiDocSyntaxKind;
}
