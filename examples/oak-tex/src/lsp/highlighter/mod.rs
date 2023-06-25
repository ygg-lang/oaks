#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};
use std::borrow::Cow;

/// TeX language highlighter
#[derive(Default, Clone)]
pub struct TexHighlighter;

impl Highlighter for TexHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // TODO: Implement real TeX highlighting logic
        Ok(HighlightResult { source: Cow::Borrowed(source), segments: Vec::new() })
    }
}

impl TexHighlighter {
    /// Creates a new TeX highlighter
    pub fn new() -> Self {
        Self
    }
}
