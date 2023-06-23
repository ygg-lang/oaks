#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

#[allow(missing_docs)]
pub struct RegexHighlighter {}

impl RegexHighlighter {
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
