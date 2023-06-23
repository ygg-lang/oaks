#![doc = include_str!("readme.md")]
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommentConfig {
    pub line_comment: Option<String>,
    pub block_comment: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringConfig {
    pub quotes: Vec<char>,
    pub escape_char: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhitespaceConfig {
    pub characters: Vec<char>,
    pub new_line_characters: Vec<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonLanguage {
    pub comment_config: CommentConfig,
    pub string_config: StringConfig,
    pub whitespace_config: WhitespaceConfig,
}

impl VonLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for VonLanguage {
    fn default() -> Self {
        Self {
            comment_config: CommentConfig { line_comment: Some("#".to_string()), block_comment: None },
            string_config: StringConfig { quotes: vec!['"', '\''], escape_char: Some('\\') },
            whitespace_config: WhitespaceConfig { characters: vec![' ', '\t'], new_line_characters: vec!['\n', '\r'] },
        }
    }
}

impl Language for VonLanguage {
    const NAME: &'static str = "von";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VonTokenType;
    type ElementType = crate::parser::element_type::VonElementType;
    type TypedRoot = crate::ast::VonRoot;
}
