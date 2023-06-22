#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

extern crate oak_core;
extern crate serde;

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

mod builder;
mod formatter;
pub mod highlighter;
pub mod lsp;
pub mod mcp;

// 重新导出主要类型
pub use crate::{ast::GoRoot, builder::GoBuilder, formatter::GoFormatter, highlighter::GoHighlighter, kind::GoSyntaxKind, language::GoLanguage, lexer::GoLexer, lsp::GoLanguageService, parser::GoParser};

pub use crate::mcp::serve_go_mcp;
