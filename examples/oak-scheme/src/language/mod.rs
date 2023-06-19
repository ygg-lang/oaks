use crate::kind::SchemeSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemeLanguage;

impl Language for SchemeLanguage {
    type SyntaxKind = SchemeSyntaxKind;
    type TypedRoot = ();
}
