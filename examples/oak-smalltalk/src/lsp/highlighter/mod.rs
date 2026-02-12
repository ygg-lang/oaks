#![doc = include_str!("readme.md")]
//! Smalltalk syntax highlighter
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};

/// Smalltalk syntax highlighter
pub struct SmalltalkHighlighter;

impl SmalltalkHighlighter {
    /// Creates a new Smalltalk highlighter instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for SmalltalkHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for SmalltalkHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // TODO: Implement real Smalltalk highlighting logic
        Ok(HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(source) })
    }
}
