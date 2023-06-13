#![no_std]

extern crate alloc;

// Public modules
pub mod builtin_rules;
pub mod comment;
pub mod config;
pub mod errors;
pub mod formatter;
pub mod rules;
pub mod visitor;

// Re-export core types
pub use builtin_rules::create_builtin_rules;
pub use comment::{Comment, CommentCollector, CommentKind, CommentProcessor};
pub use config::{FormatConfig, IndentStyle, LineEnding};
// TODO: 这些错误类型在 errors 模块中不存在
// pub use errors::{Diagnostic, DiagnosticLevel, FormatError, FormatResult, ParseError, PexError, PexErrorKind};
pub use errors::{FormatError, FormatResult};
pub use formatter::{FormatContext, FormatOutput, Formatter};
pub use rules::{BasicFormatRule, FormatRule, RuleSet};
pub use visitor::{FormatTraverser, FormatVisitor};
