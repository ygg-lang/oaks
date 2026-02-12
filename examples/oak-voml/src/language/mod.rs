//! Voml language definition.

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use oak_core::{Language, LanguageCategory};

/// Configuration for comments in Voml.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentConfig {
    /// Line comment prefix (e.g., `//`).
    pub line_comment: Option<String>,
    /// Block comment start and end (e.g., `/*` and `*/`).
    pub block_comment: Option<(String, String)>,
}

/// Configuration for strings in Voml.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringConfig {
    /// Supported quote characters (e.g., `'` and `"`).
    pub quotes: Vec<char>,
    /// Escape character (e.g., `\`).
    pub escape_char: Option<char>,
}

/// Configuration for whitespace in Voml.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhitespaceConfig {
    /// Characters considered as whitespace (e.g., space, tab).
    pub characters: Vec<char>,
    /// Characters considered as newlines.
    pub new_line_characters: Vec<char>,
}

/// The Voml language definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VomlLanguage {
    /// Comment configuration.
    pub commentconfig: CommentConfig,
    /// String configuration.
    pub stringconfig: StringConfig,
    /// Whitespace configuration.
    pub whitespaceconfig: WhitespaceConfig,
}

impl VomlLanguage {
    /// Creates a new `VomlLanguage` with default settings.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for VomlLanguage {
    fn default() -> Self {
        Self {
            commentconfig: CommentConfig { line_comment: Some("//".to_string()), block_comment: Some(("/*".to_string(), "*/".to_string())) },
            stringconfig: StringConfig { quotes: vec!['"', '\''], escape_char: Some('\\') },
            whitespaceconfig: WhitespaceConfig { characters: vec![' ', '\t'], new_line_characters: vec!['\n', '\r'] },
        }
    }
}

impl Language for VomlLanguage {
    const NAME: &'static str = "voml";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VomlTokenType;
    type ElementType = crate::parser::element_type::VomlElementType;
    type TypedRoot = crate::ast::VRoot;
}
