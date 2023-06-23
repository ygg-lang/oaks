#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Stylus support for the Oak language framework.

pub mod ast;
pub mod builder;
pub mod errors;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

mod parser;

pub use crate::{builder::StylusBuilder, language::StylusLanguage, lexer::StylusLexer, parser::StylusParser};

/// Highlighter implementation.
#[cfg(all(feature = "lsp", feature = "oak-highlight"))]
#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::StylusHighlighter;

/// Formatter implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::StylusFormatter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::StylusLanguageService;

/// MCP service implementation.
#[cfg(all(feature = "lsp", feature = "mcp"))]
#[cfg(feature = "lsp")]
pub use crate::mcp::serve_stylus_mcp;
pub use crate::{lexer::token_type::StylusTokenType as TokenType, parser::element_type::StylusElementType as ElementType};
