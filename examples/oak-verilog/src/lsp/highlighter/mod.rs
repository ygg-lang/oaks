#![doc = include_str!("readme.md")]
use crate::VerilogLanguage;
use oak_highlight::highlighter::Highlighter;

/// Highlighter implementation for Verilog.
pub struct VerilogHighlighter {}

impl Highlighter for VerilogHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: oak_highlight::themes::Theme) -> oak_core::errors::ParseResult<oak_highlight::highlighter::HighlightResult<'a>> {
        let theme_config = theme.get_theme();
        Ok(oak_highlight::highlighter::HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
