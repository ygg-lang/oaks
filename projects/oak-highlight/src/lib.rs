#![feature(new_range_api)]
#![recursion_limit = "512"]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod exporters;
pub mod highlighter;
pub mod themes;

pub use crate::{
    exporters::{AnsiExporter, CssExporter, ExportFormat, Exporter, HtmlExporter, JsonExporter},
    highlighter::{HighlightResult, HighlightSegment, HighlightStyle, HighlightTheme, Highlighter, OakHighlighter},
    themes::Theme,
};
