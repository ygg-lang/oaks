use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// SQL 高亮
pub struct SqlHighlighter;

impl Highlighter for SqlHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _language: &str, _theme: Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        todo!()
    }
}
