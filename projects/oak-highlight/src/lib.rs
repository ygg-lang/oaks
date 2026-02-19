#![feature(new_range_api)]
#![recursion_limit = "512"]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// Exporters for converting highlighted segments to different formats.
pub mod exporters;
/// Core highlighting logic and structures.
pub mod highlighter;
/// Predefined themes and theme loading logic.
pub mod themes;

pub use crate::{
    exporters::{AnsiExporter, CssExporter, ExportFormat, Exporter, HtmlExporter, JsonExporter},
    highlighter::{HighlightResult, HighlightSegment, HighlightSpan, HighlightStyle, HighlightTheme, Highlighter, OakHighlighter},
    themes::Theme,
};
