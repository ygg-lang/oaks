#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![feature(portable_simd)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Rust support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

pub use crate::{ast::RustRoot, builder::RustBuilder, language::RustLanguage, lexer::RustLexer, parser::RustParser};
pub use lexer::token_type::RustTokenType;
pub use parser::RustElementType;

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::{Highlighter, RustHighlighter};

#[cfg(feature = "lsp")]
pub use crate::lsp::RustLanguageService;

#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::RustFormatter;
