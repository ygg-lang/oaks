use crate::kind::JavaSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaLanguage;

impl Language for JavaLanguage {
    type SyntaxKind = JavaSyntaxKind;
}
