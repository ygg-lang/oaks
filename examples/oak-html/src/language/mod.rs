use crate::kind::HtmlSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone)]
pub struct HtmlLanguage {}

impl Default for HtmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for HtmlLanguage {
    type SyntaxKind = HtmlSyntaxKind;
    type TypedRoot = ();
}
