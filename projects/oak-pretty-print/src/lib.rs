#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![feature(new_range_api)]

extern crate alloc;

// Public modules
pub mod builtin_rules;
pub mod comment;
pub mod config;
pub mod document;
pub mod errors;
pub mod formatter;
pub mod rules;

// Re-export commonly used types
pub use crate::{
    builtin_rules::create_builtin_rules,
    comment::{Comment, CommentCollector, CommentKind, CommentProcessor},
    config::{FormatConfig, IndentStyle, LineEnding},
    document::Doc,
    errors::FormatResult,
    formatter::{FormatContext, FormatOutput, Formatter},
    rules::{FormatRule, RuleSet},
};
