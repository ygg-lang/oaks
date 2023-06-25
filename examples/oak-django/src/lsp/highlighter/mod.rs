#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// Django syntax highlighter.
///
/// This module provides syntax highlighting for Django source code, supporting keywords, tags, filters, comments, etc.
pub struct DjangoHighlighter;

impl DjangoHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for DjangoHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = oak_highlight::highlighter::OakHighlighter::new();
        highlighter.highlight(source, language, theme)
    }
}
