use crate::{ast::PascalRoot, kind::PascalSyntaxKind};
use oak_core::Language;

#[derive(Debug, Default)]
pub struct PascalLanguage {}

impl Language for PascalLanguage {
    type SyntaxKind = PascalSyntaxKind;
    type TypedRoot = PascalRoot;
}
