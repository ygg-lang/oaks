use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentConfig {
    pub line_comment: Option<String>,
    pub block_comment: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringConfig {
    pub quotes: Vec<char>,
    pub escape_char: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitespaceConfig {
    pub characters: Vec<char>,
    pub new_line_characters: Vec<char>,
}

pub struct VLangLanguage {
    pub comment_config: CommentConfig,
    pub string_config: StringConfig,
    pub whitespace_config: WhitespaceConfig,
}

impl Default for VLangLanguage {
    fn default() -> Self {
        Self {
            comment_config: CommentConfig { line_comment: Some("//".to_string()), block_comment: Some(("/*".to_string(), "*/".to_string())) },
            string_config: StringConfig { quotes: vec!['"', '\''], escape_char: Some('\\') },
            whitespace_config: WhitespaceConfig { characters: vec![' ', '\t'], new_line_characters: vec!['\n', '\r'] },
        }
    }
}

impl Language for VLangLanguage {
    const NAME: &'static str = "vlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VLangSyntaxKind;
    type ElementType = crate::kind::VLangSyntaxKind;
    type TypedRoot = ();
}
