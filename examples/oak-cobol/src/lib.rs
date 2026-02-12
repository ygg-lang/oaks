#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! COBOL support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

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

pub use crate::{ast::CobolRoot, language::CobolLanguage, lexer::CobolLexer, parser::CobolParser};

pub use oak_core::{ElementType, TokenType};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::CobolHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::CobolLanguageService;
/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::CobolFormatter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_cobol_mcp;
pub use lexer::token_type::CobolTokenType;
pub use parser::element_type::CobolElementType;
