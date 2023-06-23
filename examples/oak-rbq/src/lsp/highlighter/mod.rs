#![doc = include_str!("readme.md")]
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// RBQ 高亮
pub struct RbqHighlighter;

impl Highlighter for RbqHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _language: &str, _theme: Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        todo!()
    }
}
