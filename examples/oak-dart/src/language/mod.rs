use crate::{ast::DartRoot, kind::DartSyntaxKind};
use oak_core::Language;

#[derive(Debug, Default)]
pub struct DartLanguage {}

impl Language for DartLanguage {
    type SyntaxKind = DartSyntaxKind;
    type TypedRoot = DartRoot;
}
