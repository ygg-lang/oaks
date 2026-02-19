/// Jinja Language module
///
/// This module defines the language characteristics and configuration for Jinja templates.
use oak_core::language::{Language, LanguageCategory};

use crate::{ast::JinjaRoot, lexer::token_type::JinjaTokenType, parser::element_type::JinjaElementType};

/// Language definition for Jinja templates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JinjaLanguage {
    /// Whether to trim whitespace around delimiters
    pub trim_blocks: bool,
    /// Whether to lstrip blocks
    pub lstrip_blocks: bool,
    /// Whether to keep trailing newlines
    pub keep_trailing_newline: bool,
    /// Variable start delimiter
    pub variable_start: String,
    /// Variable end delimiter
    pub variable_end: String,
    /// Tag start delimiter
    pub tag_start: String,
    /// Tag end delimiter
    pub tag_end: String,
    /// Comment start delimiter
    pub comment_start: String,
    /// Comment end delimiter
    pub comment_end: String,
}

impl Default for JinjaLanguage {
    fn default() -> Self {
        Self {
            trim_blocks: false,
            lstrip_blocks: false,
            keep_trailing_newline: false,
            variable_start: "{{".to_string(),
            variable_end: "}}".to_string(),
            tag_start: "{%".to_string(),
            tag_end: "%}".to_string(),
            comment_start: "{#".to_string(),
            comment_end: "#}".to_string(),
        }
    }
}

impl Language for JinjaLanguage {
    const NAME: &'static str = "Jinja2";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = JinjaTokenType;
    type ElementType = JinjaElementType;
    type TypedRoot = JinjaRoot<'static>;
}

impl JinjaLanguage {
    /// Create a new Jinja language instance
    pub fn new() -> Self {
        Self::default()
    }
}
