#![doc = include_str!("readme.md")]
use oak_highlight::highlighter::Highlighter;

/// Highlighter implementation for CSS.
pub struct CssHighlighter {}

impl Highlighter for CssHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: oak_highlight::themes::Theme) -> oak_core::errors::ParseResult<oak_highlight::highlighter::HighlightResult<'a>> {
        let _theme_config = theme.get_theme();
        Ok(oak_highlight::highlighter::HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
