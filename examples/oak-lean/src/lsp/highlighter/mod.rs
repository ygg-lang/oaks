#![doc = include_str!("readme.md")]
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

pub struct LeanHighlighter;

impl Highlighter for LeanHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        // TODO: Implement actual highlighting
        Ok(HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(source) })
    }
}
