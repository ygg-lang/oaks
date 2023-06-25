#![doc = include_str!("readme.md")]
use crate::{ast::TwigRoot, lexer::TwigLexer, parser::TwigParser};
use oak_core::{Language, LanguageCategory};

/// Twig template engine configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TwigMode {
    #[default]
    Template,
    Expression,
    // Other possible modes
}

/// Twig language definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigLanguage {
    pub allow_raw_blocks: bool,
    pub allow_custom_tags: bool,
    pub mode: TwigMode,
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
}

impl Default for TwigLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl Language for TwigLanguage {
    const NAME: &'static str = "twig";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::TwigTokenType;
    type ElementType = crate::parser::element_type::TwigElementType;
    type TypedRoot = TwigRoot;
}

impl TwigLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self {
            allow_raw_blocks: true,
            allow_custom_tags: false,
            mode: TwigMode::Template,
            variable_start: "{{".to_string(),
            variable_end: "}}".to_string(),
            tag_start: "{%".to_string(),
            tag_end: "%}".to_string(),
            comment_start: "{#".to_string(),
            comment_end: "#}".to_string(),
        }
    }

    pub fn lexer(&self) -> TwigLexer<'_> {
        TwigLexer::new(self)
    }

    pub fn parser(&self) -> TwigParser<'_> {
        TwigParser::new(self)
    }
}
