#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Prolog support for the Oak language framework.

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

pub use crate::{ast::PrologRoot, builder::PrologBuilder, language::PrologLanguage, lexer::token_type::PrologTokenType, parser::PrologParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, PrologHighlighter};

#[cfg(feature = "lsp")]
pub use crate::lsp::PrologLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_prolog_mcp;
pub use parser::element_type::PrologElementType;
