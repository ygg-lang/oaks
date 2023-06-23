#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Dot support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Kind definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;

#[cfg(feature = "mcp")]
pub mod mcp;
/// Parser module.
pub mod parser;

pub use crate::{builder::DotBuilder, language::DotLanguage, lexer::DotLexer, parser::DotParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DotHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::DotLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_dot_mcp;
pub use lexer::token_type::DotTokenType;
pub use parser::element_type::DotElementType;
