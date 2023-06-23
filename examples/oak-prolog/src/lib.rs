#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Prolog support for the Oak language framework.

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

pub use crate::{ast::PrologRoot, builder::PrologBuilder, language::PrologLanguage, lexer::token_type::PrologTokenType, parser::PrologParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, PrologHighlighter};

#[cfg(feature = "lsp")]
pub use crate::lsp::PrologLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_prolog_mcp;
pub use parser::element_type::PrologElementType;
