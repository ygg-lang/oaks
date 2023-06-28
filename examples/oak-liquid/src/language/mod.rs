/// Liquid Language module
///
/// This module defines the language characteristics and configuration for Liquid templates.
use oak_core::language::{Language, LanguageCategory};

use crate::{ast::LiquidRoot, lexer::token_type::LiquidTokenType, parser::element_type::LiquidElementType};

/// Language definition for Liquid templates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiquidLanguage {
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

impl Default for LiquidLanguage {
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

impl Language for LiquidLanguage {
    const NAME: &'static str = "Liquid2";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = LiquidTokenType;
    type ElementType = LiquidElementType;
    type TypedRoot = LiquidRoot<'static>;
}

impl LiquidLanguage {
    /// Create a new Liquid language instance
    pub fn new() -> Self {
        Self::default()
    }
}
