#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Zig support for the Oak language framework.

/// AST module.
// pub mod ast;
/// Builder module.
// // pub mod builder;
// /// Formatter module.
// // pub mod formatter;
// /// Highlighter module.
// #[cfg(feature = "oak-highlight")]
// pub mod highlighter;
/// Kind definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(feature = "lsp")]
pub mod lsp;
/// Parser module.
pub mod parser;

pub use crate::{language::ZigLanguage, lexer::ZigLexer, parser::ZigParser};
pub use oak_core::{ElementType, TokenType};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::ZigLanguageService;

// /// MCP service implementation.
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_zig_mcp;
pub use lexer::token_type::ZigTokenType;
pub use parser::element_type::ZigElementType;
