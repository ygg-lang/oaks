#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Scheme support for the Oak language framework.

pub mod language;
pub mod lexer;
pub mod parser;

mod builder;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{builder::SchemeBuilder, language::SchemeLanguage, lexer::SchemeLexer, parser::SchemeParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::SchemeLanguageService;
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::SchemeFormatter;
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::SchemeHighlighter;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_scheme_mcp;
pub use lexer::token_type::SchemeTokenType;
pub use parser::element_type::SchemeElementType;
