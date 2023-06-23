#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Objective-c support for the Oak language framework.

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

pub use crate::{ast::ObjectiveCRoot, builder::ObjectiveCBuilder, language::ObjectiveCLanguage, lexer::ObjectiveCLexer, parser::ObjectiveCParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, ObjectiveCHighlighter};

#[cfg(feature = "lsp")]
pub use crate::lsp::ObjectiveCLanguageService;

// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_objective_c_mcp;
pub use lexer::token_type::ObjectiveCTokenType;
pub use parser::element_type::ObjectiveCElementType;
