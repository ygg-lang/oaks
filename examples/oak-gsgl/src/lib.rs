#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Gsgl support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Highlighter module.
#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter;
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
/// Syntax module.
pub use crate::ast::GsglRoot;
pub use crate::{builder::GsglBuilder, language::GsglLanguage, lexer::GsglLexer, parser::GsglParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::GsglHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::GsglLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_gsgl_mcp;
pub use lexer::token_type::GsglTokenType;
pub use parser::element_type::GsglElementType;
