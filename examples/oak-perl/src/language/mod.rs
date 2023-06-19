use crate::{ast::PerlRoot, kind::PerlSyntaxKind};
use oak_core::Language;

pub type TypedRoot = PerlRoot;

#[derive(Debug, Default, Clone, Copy)]
pub struct PerlLanguage;

impl Language for PerlLanguage {
    type SyntaxKind = PerlSyntaxKind;
    type TypedRoot = ();
}
