#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Racket support for the Oak language framework.

pub mod language;
pub mod lexer;
pub mod parser;

mod builder;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{builder::RacketBuilder, language::RacketLanguage, lexer::RacketLexer, parser::RacketParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::RacketLanguageService;
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::RacketFormatter;
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::RacketHighlighter;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_racket_mcp;
pub use lexer::token_type::RacketTokenType;
pub use parser::element_type::RacketElementType;
