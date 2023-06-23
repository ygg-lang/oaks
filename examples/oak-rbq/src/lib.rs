#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Rbq support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Syntax kind module.
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
/// Semantic tokens module.
#[cfg(feature = "lsp")]
pub mod semantic_tokens;

pub use crate::{
    ast::RbqRoot,
    builder::RbqBuilder,
    language::RbqLanguage,
    lexer::{RbqLexer, token_type::RbqTokenType},
    parser::{RbqParser, element_type::RbqElementType},
};

/// Alias for RbqTokenType to support tests and common usage
pub type RbqSyntaxKind = RbqTokenType;

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::RbqHighlighter;

/// Formatter implementation.
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::RbqFormatter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::RbqLanguageService;

/// MCP implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_rbq_mcp;
