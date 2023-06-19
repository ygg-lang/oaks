use crate::kind::CssSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CssLanguage;

impl Language for CssLanguage {
    type SyntaxKind = CssSyntaxKind;
    type TypedRoot = ();
}
