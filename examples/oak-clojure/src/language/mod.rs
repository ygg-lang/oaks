use crate::{lexer::ClojureTokenType, parser::ClojureElementType};
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

    type TokenType = ClojureTokenType;
    type ElementType = ClojureElementType;
    type TypedRoot = ();
}
