#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Purescript support for the Oak language framework.

pub mod ast;
pub mod builder;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use crate::{ast::PurescriptRoot, builder::PurescriptBuilder, language::PurescriptLanguage, lexer::PurescriptLexer, parser::PurescriptParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, PurescriptHighlighter};

/// MCP service implementation.
// #[cfg(feature = "mcp")]
// pub mod mcp;
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_purescript_mcp;
pub use parser::element_type::PurescriptElementType;
