#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Rbq support for the Oak language framework.
//!
//! This crate provides lexing, parsing, AST generation, and LSP support for the RBQ language.

/// AST module containing node definitions for the RBQ language.
pub mod ast;
/// Builder module for constructing RBQ trees.
pub mod builder;

/// Language configuration and syntax kind definitions.
pub mod language;
/// Lexer implementation for RBQ.
pub mod lexer;
/// LSP-related functionality (hover, completion, highlighting).
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP (Model Context Protocol) integration for RBQ.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser implementation for RBQ.
pub mod parser;

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
