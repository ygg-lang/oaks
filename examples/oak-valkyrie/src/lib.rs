#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Valkyrie support for the Oak language framework.

/// AST definitions for Valkyrie.
pub mod ast;
/// Builder implementation for Valkyrie.
pub mod builder;
/// Language definition for Valkyrie.
pub mod language;
/// Lexer implementation for Valkyrie.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;
/// Parser implementation for Valkyrie.
pub mod parser;

// Re-export main types for convenience
pub use crate::{builder::ValkyrieBuilder, language::ValkyrieLanguage, lexer::ValkyrieLexer, parser::ValkyrieParser};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::ValkyrieLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::ValkyrieFormatter;

/// Re-export lexer types
pub mod lexer_types {
    pub use crate::lexer::ValkyrieKeywords;
}
pub use lexer_types::*;

#[cfg(feature = "oak-highlight")]
/// Highlighter implementation.
pub use crate::lsp::highlighter::ValkyrieHighlighter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_valkyrie_mcp;
pub use crate::{lexer::token_type::ValkyrieSyntaxKind as TokenType, parser::element_type::ValkyrieElementType as ElementType};
