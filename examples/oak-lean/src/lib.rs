#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Lean support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

pub use crate::{
    ast::LeanRoot,
    builder::LeanBuilder,
    language::LeanLanguage,
    lexer::{LeanLexer, token_type::LeanTokenType},
    parser::{LeanParser, element_type::LeanElementType},
};

// Highlighter implementation.
// #[cfg(feature = "oak-highlight")]
// pub use crate::lsp::highlighter::LeanHighlighter;
//
// LSP implementation.
// #[cfg(feature = "lsp")]
// pub use crate::{ lsp::LeanLanguageService};
//
// MCP service implementation.
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_lean_mcp;
