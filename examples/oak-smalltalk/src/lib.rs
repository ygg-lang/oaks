#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Smalltalk support for the Oak language framework.

mod builder;
// // mod formatter;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

pub use crate::{builder::SmalltalkBuilder, language::SmalltalkLanguage, lexer::SmalltalkLexer, parser::SmalltalkParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::SmalltalkHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::SmalltalkFormatter;
// // pub use crate::{ lsp::SmalltalkLanguageService};
// #[cfg(feature = "mcp")]
// #[cfg(feature = "lsp")]
// pub use crate::mcp::serve_smalltalk_mcp;
pub use lexer::token_type::SmalltalkTokenType;
pub use parser::element_type::SmalltalkElementType;
