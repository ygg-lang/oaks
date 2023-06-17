use crate::kind::ClojureSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClojureLanguage;

impl Language for ClojureLanguage {
    type SyntaxKind = ClojureSyntaxKind;
}
