#![doc = include_str!("readme.md")]
use crate::ast::OCamlRoot;
use oak_core::{Language, LanguageCategory};

/// The OCaml language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OCamlLanguage {}

impl OCamlLanguage {
    /// Create a new OCaml language definition.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for OCamlLanguage {
    const NAME: &'static str = "ocaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::OCamlTokenType;
    type ElementType = crate::parser::element_type::OCamlElementType;
    type TypedRoot = OCamlRoot;
}
