use crate::kind::MarkdownSyntaxKind;
use oak_core::Language;

pub struct MarkdownLanguage {}

impl Language for MarkdownLanguage {
    type SyntaxKind = MarkdownSyntaxKind;
}
