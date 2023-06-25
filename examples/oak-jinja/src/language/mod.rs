use oak_core::{Language, LanguageCategory};

/// Jinja2 template language configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JinjaLanguage {
    /// Variable tag start
    pub variable_start: String,
    /// Variable tag end
    pub variable_end: String,
    /// Tag start
    pub tag_start: String,
    /// Tag end
    pub tag_end: String,
    /// Comment start
    pub comment_start: String,
    /// Comment end
    pub comment_end: String,
    /// Line statement prefix
    pub line_statement_prefix: Option<String>,
    /// Line comment prefix
    pub line_comment_prefix: Option<String>,
}

impl JinjaLanguage {
    /// Creates a new Jinja2 language instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for JinjaLanguage {
    fn default() -> Self {
        Self {
            variable_start: "{{".to_string(),
            variable_end: "}}".to_string(),
            tag_start: "{%".to_string(),
            tag_end: "%}".to_string(),
            comment_start: "{#".to_string(),
            comment_end: "#}".to_string(),
            line_statement_prefix: None,
            line_comment_prefix: None,
        }
    }
}

impl Language for JinjaLanguage {
    const NAME: &'static str = "jinja";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::JinjaTokenType;
    type ElementType = crate::parser::element_type::JinjaElementType;
    type TypedRoot = (); // TypedRoot is not defined yet
}
