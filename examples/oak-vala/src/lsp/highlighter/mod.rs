#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, OakHighlighter, themes::Theme};

/// Vala 语法高亮器
pub struct ValaHighlighter;

impl Highlighter for ValaHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new().theme(theme);
        highlighter.highlight(source, language, theme)
    }
}
