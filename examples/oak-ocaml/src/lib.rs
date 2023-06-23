#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Ocaml support for the Oak language framework.

pub mod ast;
pub mod builder;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

pub use builder::OCamlBuilder;
pub use language::OCamlLanguage;
pub use lexer::OCamlLexer;
pub use parser::OCamlParser;

/// Highlighter implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, OCamlHighlighter};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::OCamlLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_ocaml_mcp;
pub use lexer::token_type::OCamlTokenType;
pub use parser::element_type::OCamlElementType;
