#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! D support for the Oak language framework.

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
/// Parser module.
pub mod parser;

// MCP module.
// #[cfg(feature = "mcp")]
// pub mod mcp;

pub use crate::{ast::DRoot, builder::DBuilder, language::DLanguage, lexer::DLexer, parser::DParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::DLanguageService;
/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::DFormatter;

// MCP service implementation.
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_d_mcp;
pub use lexer::token_type::DTokenType;
pub use parser::element_type::DElementType;
