#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

// Public modules
pub mod builtin_rules;
pub mod comment;
pub mod config;
pub mod errors;
pub mod formatter;
pub mod rules;
pub mod visitor;

// Re-export commonly used types
pub use crate::{
    builtin_rules::create_builtin_rules,
    comment::{Comment, CommentCollector, CommentKind, CommentProcessor},
    config::{FormatConfig, IndentStyle, LineEnding},
    errors::{FormatError, FormatResult},
    formatter::{FormatContext, FormatOutput, Formatter},
    rules::{BasicFormatRule, FormatRule, RuleSet},
    visitor::{FormatTraverser, FormatVisitor},
};
