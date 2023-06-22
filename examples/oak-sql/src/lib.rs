#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;
// pub mod syntax;

mod formatter;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;

// 重新导出主要类型
pub use crate::{builder::SqlBuilder, formatter::SqlFormatter, highlighter::SqlHighlighter, kind::SqlSyntaxKind, language::SqlLanguage, lexer::SqlLexer, lsp::SqlLanguageService};
