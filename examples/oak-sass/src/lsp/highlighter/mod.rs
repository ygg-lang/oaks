#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// Sass highlighter
pub struct SassHighlighter {}

impl SassHighlighter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Highlighter for SassHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // Temporarily return a simple result
        Ok(HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
