#![doc = include_str!("readme.md")]
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};
use std::borrow::Cow;

/// RBQ highlighter.
pub struct RbqHighlighter;

impl Highlighter for RbqHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        Ok(HighlightResult { segments: Vec::new(), source: Cow::Borrowed(source) })
    }
}
