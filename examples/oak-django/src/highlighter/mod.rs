use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// Django 高亮
pub struct DjangoHighlighter {
    pub use_parser: bool,
}

impl DjangoHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for DjangoHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = oak_highlight::highlighter::OakHighlighter::new();
        highlighter.highlight(source, language, theme)
    }
}
