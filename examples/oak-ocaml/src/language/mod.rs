use crate::{ast::OCamlRoot, kind::OCamlSyntaxKind};
use oak_core::Language;

#[derive(Debug, Clone)]
pub struct OCamlLanguage;

impl Language for OCamlLanguage {
    type SyntaxKind = OCamlSyntaxKind;
    type TypedRoot = OCamlRoot;
}
