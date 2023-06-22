use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Theme, highlighter::Highlighter};

/// Scala 高亮
pub struct ScalaHighlighter {
    _use_parser: bool,
}

impl Highlighter for ScalaHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _theme: &str, _theme_config: Theme) -> ParseResult<HighlightResult<'a>> {
        todo!()
    }
}
