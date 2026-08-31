#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Theme, highlighter::Highlighter};
use std::borrow::Cow;

/// Scala highlighter.
pub struct ScalaHighlighter {
    _use_parser: bool,
}

impl Highlighter for ScalaHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        Ok(HighlightResult { segments: Vec::new(), source: Cow::Borrowed(source) })
    }
}
