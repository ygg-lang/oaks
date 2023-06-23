#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Javadoc support for the Oak language framework.

/// AST module.
pub mod ast;
mod builder;
/// Kind module.
/// Language module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP and related services (HighlighterMCP).
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "mcp"))]
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

pub use crate::{ast::JavadocRoot, builder::JavadocBuilder, language::JavadocLanguage, lexer::JavadocLexer, parser::JavadocParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::JavadocHighlighter;

/// LSP service implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::JavadocLanguageService;
pub use lexer::token_type::JavadocTokenType;
pub use parser::element_type::JavadocElementType;
