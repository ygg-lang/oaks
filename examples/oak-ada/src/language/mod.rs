use crate::kind::AdaSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdaLanguage;

impl Language for AdaLanguage {
    type SyntaxKind = AdaSyntaxKind;
}
