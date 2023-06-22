use crate::ast::OCamlRoot;
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone)]
pub struct OCamlLanguage;

impl Language for OCamlLanguage {
    const NAME: &'static str = "ocaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::OCamlSyntaxKind;
    type ElementType = crate::kind::OCamlSyntaxKind;
    type TypedRoot = OCamlRoot;
}
