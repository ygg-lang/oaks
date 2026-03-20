use crate::language::LiquidLanguage;
/// Highlighter module for Liquid
///
/// This module provides syntax highlighting support for Liquid templates.
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};

/// Highlighter for Liquid templates
#[derive(Debug, Clone)]
pub struct LiquidHighlighter {
    theme: Theme,
}

impl LiquidHighlighter {
    /// Creates a new Liquid highlighter with the given theme
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

impl Highlighter for LiquidHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> Result<HighlightResult<'a>, oak_core::errors::OakError> {
        let theme_config = theme.get_theme();
        let segments = vec![oak_highlight::HighlightSegment { span: oak_highlight::HighlightSpan { start: 0, end: source.len() }, style: theme_config.resolve_style("none"), text: std::borrow::Cow::Borrowed(source) }];
        Ok(HighlightResult { segments, source: std::borrow::Cow::Borrowed(source) })
    }
}
