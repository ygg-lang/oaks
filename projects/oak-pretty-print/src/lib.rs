#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![feature(new_range_api)]
pub extern crate alloc;
extern crate self as oak_pretty_print;

#[doc(hidden)]
pub mod __private {
    pub use crate::alloc;
}

// Public modules
pub mod builtin_rules;
pub mod comment;
pub mod config;
pub mod document;
pub mod errors;
pub mod formatter;
pub mod rules;
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
pub type Doc<'a> = Document<'a>;

pub use oak_macros::{AsDocument, FormatRule, define_rules, doc};

/// 空文档
pub const NIL: Document<'static> = Document::Nil;
/// 强制换行
pub const LINE: Document<'static> = Document::Line;

/// 增加缩进
pub fn indent<'a>(doc: Document<'a>) -> Document<'a> {
    Document::indent(doc)
}

/// 将多个文档用指定分隔符连接
pub fn join<'a>(docs: impl IntoIterator<Item = Document<'a>>, separator: Document<'a>) -> Document<'a> {
    Document::join(docs, separator)
}
/// 软换行
pub const SOFT_LINE: Document<'static> = Document::SoftLine;
/// 软换行（带空格）
pub const SOFT_LINE_SPACE: Document<'static> = Document::SoftLineSpace;
/// 强制换行且会导致父级 Group 也展开
pub const HARD_LINE: Document<'static> = Document::HardLine;
