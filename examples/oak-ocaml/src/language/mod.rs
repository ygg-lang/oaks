use crate::ast::OCamlRoot;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct OCamlLanguage {}

impl OCamlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for OCamlLanguage {
    const NAME: &'static str = "ocaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::OCamlSyntaxKind;
    type ElementType = crate::kind::OCamlSyntaxKind;
    type TypedRoot = OCamlRoot;
}
