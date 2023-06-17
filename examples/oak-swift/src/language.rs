use crate::kind::SwiftSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwiftLanguage;

impl Language for SwiftLanguage {
    type SyntaxKind = SwiftSyntaxKind;
}
