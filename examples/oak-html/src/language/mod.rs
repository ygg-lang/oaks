use crate::kind::HtmlSyntaxKind;
use oak_core::Language;

pub struct HtmlLanguage {}

impl Language for HtmlLanguage {
    type SyntaxKind = HtmlSyntaxKind;
}
