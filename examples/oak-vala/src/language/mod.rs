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

pub struct ValaLanguage {
    pub comment_config: CommentConfig,
    pub string_config: StringConfig,
    pub whitespace_config: WhitespaceConfig,
}

impl Default for ValaLanguage {
    fn default() -> Self {
        Self {
            comment_config: CommentConfig { line_comment: Some("//".to_string()), block_comment: Some(("/*".to_string(), "*/".to_string())) },
            string_config: StringConfig { quotes: vec!['"', '\''], escape_char: Some('\\') },
            whitespace_config: WhitespaceConfig { characters: vec![' ', '\t'], new_line_characters: vec!['\n', '\r'] },
        }
    }
}

impl Language for ValaLanguage {
    const NAME: &'static str = "vala";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ValaSyntaxKind;
    type ElementType = crate::kind::ValaSyntaxKind;
    type TypedRoot = crate::ast::ValaRoot;
}
