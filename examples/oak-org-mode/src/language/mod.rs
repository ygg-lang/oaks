use oak_core::{Language, LanguageCategory};
use std::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[derive(Debug, Clone)]
pub struct OrgModeLanguage {
    pub todo_keywords: Vec<String>,
    pub done_keywords: Vec<String>,
    pub strict_mode: bool,
}

impl OrgModeLanguage {
    pub fn new() -> Self {
        Self { todo_keywords: vec!["TODO".to_string(), "NEXT".to_string(), "WAITING".to_string()], done_keywords: vec!["DONE".to_string(), "CANCELLED".to_string()], strict_mode: false }
    }

    pub fn with_todo_keywords(mut self, keywords: Vec<String>) -> Self {
        self.todo_keywords = keywords;
        self
    }

    pub fn with_done_keywords(mut self, keywords: Vec<String>) -> Self {
        self.done_keywords = keywords;
        self
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
}

impl Default for OrgModeLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for OrgModeLanguage {
    const NAME: &'static str = "org-mode";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::OrgModeSyntaxKind;
    type ElementType = crate::kind::OrgModeSyntaxKind;
    type TypedRoot = ();
}
