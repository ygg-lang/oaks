use crate::kind::XmlSyntaxKind;
use oak_core::Language;

pub struct XmlLanguage {}

impl Language for XmlLanguage {
    type SyntaxKind = XmlSyntaxKind;
}
