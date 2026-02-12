#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Theme, highlighter::Highlighter};

/// Scala highlighter
pub struct ScalaHighlighter {
    _use_parser: bool,
}

impl Highlighter for ScalaHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _theme: &str, _themeconfig: Theme) -> ParseResult<HighlightResult<'a>> {
        todo!()
    }
}
