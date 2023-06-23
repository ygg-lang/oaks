#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Dejavu support for the Oak language framework.

/// AST definitions for Dejavu.
pub mod ast;
/// Builder implementation for Dejavu.
pub mod builder;
/// Language definition for Dejavu.
pub mod language;
/// Lexer implementation for Dejavu.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;
/// Parser implementation for Dejavu.
pub mod parser;

// Re-export main types for convenience
pub use crate::{builder::DejavuBuilder, language::DejavuLanguage, lexer::DejavuLexer, parser::DejavuParser};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::DejavuLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::DejavuFormatter;

/// Re-export lexer types
pub mod lexer_types {
    pub use crate::lexer::DejavuKeywords;
}
pub use lexer_types::*;

#[cfg(feature = "oak-highlight")]
/// Highlighter implementation.
pub use crate::lsp::highlighter::DejavuHighlighter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_dejavu_mcp;
pub use crate::{lexer::token_type::DejavuSyntaxKind as TokenType, parser::element_type::DejavuElementType as ElementType};
