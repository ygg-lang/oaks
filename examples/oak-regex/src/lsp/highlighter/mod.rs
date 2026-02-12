#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

/// Syntax highlighter for regular expressions.
pub struct RegexHighlighter {}

impl RegexHighlighter {
    /// Create a new instance of the regular expression highlighter.
    pub fn new() -> Self {
        Self {}
    }
}

impl Highlighter for RegexHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();
        highlighter.highlight(source, language, theme)
    }
}
