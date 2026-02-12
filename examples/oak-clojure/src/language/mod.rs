#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClojureLanguage {}

impl ClojureLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ClojureLanguage {
    const NAME: &'static str = "clojure";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ClojureTokenType;
    type ElementType = crate::parser::element_type::ClojureElementType;
    type TypedRoot = ();
}
