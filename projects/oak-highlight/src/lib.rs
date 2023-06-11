#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![no_std]

extern crate alloc;

pub mod errors;
pub mod exporters;
pub mod highlighter;
pub mod themes;

pub use crate::{
    exporters::{AnsiExporter, CssExporter, ExportFormat, Exporter, HtmlExporter as ExporterHtmlExporter, JsonExporter},
    highlighter::{HighlightResult, HighlightSegment, HighlightStyle, HighlightTheme, Highlighter, HtmlExporter},
    themes::*,
};
