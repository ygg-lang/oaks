#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// SCSS highlighter
pub struct ScssHighlighter;

impl ScssHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for ScssHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // Temporarily return a simple result
        Ok(HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
