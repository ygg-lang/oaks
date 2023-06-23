#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! Pretty-printing framework for the Oak language framework.
//!
//! This crate provides a document-based pretty-printing system that handles
//! indentation, line breaks, and grouping for language formatters.
pub extern crate alloc;
extern crate self as oak_pretty_print;

#[doc(hidden)]
pub mod __private {
    pub use crate::alloc;
}

// Public modules
/// Built-in formatting rules
pub mod builtin_rules;
/// Comment handling and processing
pub mod comment;
/// Formatting configuration
pub mod config;
/// Document abstraction and printer
pub mod document;
/// Error types for formatting
pub mod errors;
/// Generic formatter implementation
pub mod formatter;
/// Formatting rule traits and sets
pub mod rules;
/// Traits for converting types to documents
pub mod to_doc;

// Re-export commonly used types
pub use crate::{
    builtin_rules::create_builtin_rules,
    comment::{Comment, CommentCollector, CommentKind, CommentProcessor},
    config::{FormatConfig, IndentStyle, LineEnding},
    errors::FormatResult,
    formatter::{FormatContext, FormatOutput, Formatter},
    rules::{FormatRule, RuleSet},
    to_doc::{AsDocument, ToDocument},
};
pub use oak_core::language::Language;

pub use crate::document::Document;
/// Type alias for Document
pub type Doc<'a> = Document<'a>;

pub use oak_macros::{AsDocument, FormatRule, define_rules, doc};

/// Empty document
pub const NIL: Document<'static> = Document::Nil;
/// Force a line break
pub const LINE: Document<'static> = Document::Line;

/// Increase indentation
pub fn indent<'a>(doc: Document<'a>) -> Document<'a> {
    Document::indent(doc)
}

/// Join multiple documents with a specified separator
pub fn join<'a>(docs: impl IntoIterator<Item = Document<'a>>, separator: Document<'a>) -> Document<'a> {
    Document::join(docs, separator)
}
/// Soft line break: a line break if the group breaks, otherwise empty
pub const SOFT_LINE: Document<'static> = Document::SoftLine;
/// Soft line break with space: a line break if the group breaks, otherwise a space
pub const SOFT_LINE_SPACE: Document<'static> = Document::SoftLineSpace;
/// Force a line break and propagate it to parent groups
pub const HARD_LINE: Document<'static> = Document::HardLine;
