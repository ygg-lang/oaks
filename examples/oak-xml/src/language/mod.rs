use crate::kind::XmlSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone)]
pub struct XmlLanguage {}

impl Default for XmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for XmlLanguage {
    type SyntaxKind = XmlSyntaxKind;
    type TypedRoot = ();
}
