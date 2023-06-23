#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Regex support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Formatter module.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter;
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

pub use crate::{ast::RegexRoot, builder::RegexBuilder, language::RegexLanguage, lexer::RegexLexer, parser::RegexParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::RegexHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::RegexLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::RegexFormatter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_regex_mcp;
pub use lexer::token_type::RegexTokenType;
pub use parser::element_type::RegexElementType;
