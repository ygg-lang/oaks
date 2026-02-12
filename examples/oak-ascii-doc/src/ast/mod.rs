#![doc = include_str!("readme.md")]
use core::range::Range;
use std::{string::String, vec::Vec};

/// Root node of an AsciiDoc document.
#[derive(Debug, PartialEq, Clone)]
pub struct AsciiDocRoot {
    /// List of elements in the document.
    pub elements: Vec<Element>,
}

/// Represents an element in an AsciiDoc document.
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    /// A header (e.g., `= Header 1`).
    Header {
        /// Level of the header (1 to 6).
        level: u8,
        /// Text content of the header.
        text: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// Plain text.
    Text {
        /// Content of the text.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// Bold text.
    Bold {
        /// Content of the bold text.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// Italic text.
    Italic {
        /// Content of the italic text.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// Monospace text.
    Monospace {
        /// Content of the monospace text.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// A code block.
    CodeBlock {
        /// Content of the code block.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// A hyperlink.
    Link {
        /// Target URL.
        url: String,
        /// Optional display text.
        text: Option<String>,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// A list item.
    ListItem {
        /// Content of the list item.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
    /// A comment.
    Comment {
        /// Content of the comment.
        content: String,
        /// Byte range in the source text.
        span: Range<usize>,
    },
}
