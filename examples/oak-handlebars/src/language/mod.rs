use crate::kind::HandlebarsSyntaxKind;
use oak_core::Language;

pub struct HandlebarsLanguage {}

impl Language for HandlebarsLanguage {
    type SyntaxKind = HandlebarsSyntaxKind;
}
