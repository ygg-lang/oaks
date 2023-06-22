#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

pub use crate::{ast::TclRoot, builder::TclBuilder, formatter::TclFormatter, highlighter::TclHighlighter, language::TclLanguage, lexer::TclLexer, lsp::TclLanguageService, parser::TclParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_tcl_mcp;
