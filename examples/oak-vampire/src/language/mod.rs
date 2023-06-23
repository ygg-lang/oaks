#![doc = include_str!("readme.md")]
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
pub struct VampireLanguage {
    pub comment_config: CommentConfig,
    pub string_config: StringConfig,
    pub whitespace_config: WhitespaceConfig,
}

impl Default for VampireLanguage {
    fn default() -> Self {
        Self {
            comment_config: CommentConfig { line_comment: Some("%".to_string()), block_comment: Some(("/*".to_string(), "*/".to_string())) },
            string_config: StringConfig { quotes: vec!['"', '\''], escape_char: Some('\\') },
            whitespace_config: WhitespaceConfig { characters: vec![' ', '\t'], new_line_characters: vec!['\n', '\r'] },
        }
    }
}

impl Language for VampireLanguage {
    const NAME: &'static str = "vampire";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VampireTokenType;
    type ElementType = crate::parser::element_type::VampireElementType;
    type TypedRoot = crate::ast::VampireRoot;
}
