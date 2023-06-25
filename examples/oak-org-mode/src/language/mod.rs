#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
use std::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// Org-mode language definition.
#[derive(Debug, Clone)]
pub struct OrgModeLanguage {
    /// TODO keywords list.
    pub todo_keywords: Vec<String>,
    /// DONE keywords list.
    pub done_keywords: Vec<String>,
    /// Whether to enable strict mode.
    pub strict_mode: bool,
}

impl OrgModeLanguage {
    /// Creates a new `OrgModeLanguage`.
    pub fn new() -> Self {
        Self { todo_keywords: vec!["TODO".to_string(), "NEXT".to_string(), "WAITING".to_string()], done_keywords: vec!["DONE".to_string(), "CANCELLED".to_string()], strict_mode: false }
    }

    /// Sets TODO keywords.
    pub fn with_todo_keywords(mut self, keywords: Vec<String>) -> Self {
        self.todo_keywords = keywords;
        self
    }

    /// Sets DONE keywords.
    pub fn with_done_keywords(mut self, keywords: Vec<String>) -> Self {
        self.done_keywords = keywords;
        self
    }

    /// Sets strict mode.
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
}

impl Default for OrgModeLanguage {
    fn default() -> Self {
        Self { todo_keywords: vec!["TODO".to_string(), "NEXT".to_string(), "WAITING".to_string()], done_keywords: vec!["DONE".to_string(), "CANCELLED".to_string()], strict_mode: false }
    }
}

impl Language for OrgModeLanguage {
    const NAME: &'static str = "org-mode";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::OrgModeTokenType;
    type ElementType = crate::parser::element_type::OrgModeElementType;
    type TypedRoot = ();
}
