use crate::language::JinjaLanguage;
/// Highlighter module for Jinja
///
/// This module provides syntax highlighting support for Jinja templates.
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};

/// Highlighter for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaHighlighter {
    theme: Theme,
}

impl JinjaHighlighter {
    /// Creates a new Jinja highlighter with the given theme
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

impl Highlighter for JinjaHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> Result<HighlightResult<'a>, oak_core::errors::OakError> {
        let theme_config = theme.get_theme();
        let segments = vec![oak_highlight::HighlightSegment { span: oak_highlight::HighlightSpan { start: 0, end: source.len() }, style: theme_config.resolve_style("none"), text: std::borrow::Cow::Borrowed(source) }];
        Ok(HighlightResult { segments, source: std::borrow::Cow::Borrowed(source) })
    }
}
