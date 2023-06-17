#![allow(unused)]

use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavadocLanguage;

impl Language for JavadocLanguage {
    type SyntaxKind = crate::kind::JavadocSyntaxKind;
}
