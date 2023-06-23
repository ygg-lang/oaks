#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Pascal support for the Oak language framework.

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

pub use crate::{ast::PascalRoot, builder::PascalBuilder, language::PascalLanguage, lexer::PascalLexer, parser::PascalParser};

#[cfg(feature = "lsp")]
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::PascalHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::PascalLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_pascal_mcp;
pub use lexer::token_type::PascalTokenType;
pub use parser::element_type::PascalElementType;
