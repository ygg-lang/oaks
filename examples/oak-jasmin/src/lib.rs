#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Jasmin support for the Oak language framework.

pub mod ast;
pub mod builder;

mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

mod parser;

pub use crate::{ast::JasminRoot, builder::JasminBuilder, language::JasminLanguage, lexer::JasminLexer, parser::JasminParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::JasminHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::JasminLanguageService;
pub use lexer::token_type::JasminTokenType;
pub use parser::element_type::JasminElementType;
