#![doc = include_str!("readme.md")]
use crate::language::SolidityLanguage;
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct SolidityHighlighter;

impl Highlighter for SolidityHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();
        highlighter.highlight(source, SolidityLanguage::NAME, theme)
    }
}
