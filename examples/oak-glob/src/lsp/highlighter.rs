use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};
use std::borrow::Cow;

/// Highlighter for glob pattern syntax.
#[derive(Default, Clone)]
pub struct GlobHighlighter;

impl Highlighter for GlobHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        Ok(HighlightResult { source: Cow::Borrowed(source), segments: Vec::new() })
    }
}
