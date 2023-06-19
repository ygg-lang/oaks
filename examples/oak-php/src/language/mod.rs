use crate::kind::PhpSyntaxKind;
use oak_core::{Language, SyntaxKind};

pub type TypedRoot = crate::ast::PhpRoot;

#[derive(Debug, Default)]
pub struct PhpLanguage;

impl Language for PhpLanguage {
    type SyntaxKind = PhpSyntaxKind;
    type TypedRoot = TypedRoot;
}
