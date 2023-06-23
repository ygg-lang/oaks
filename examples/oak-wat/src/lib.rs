#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Wat support for the Oak language framework.

/// AST module.
pub mod ast;
// pub mod builder;
//
// pub mod formatter;
//
// pub mod highlighter;
/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
// /// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
// pub mod mcp;
/// Parser module.
pub mod parser;

pub use crate::{ast::WatRoot, language::WatLanguage, lexer::WatLexer, parser::WatParser};

//
#[cfg(feature = "oak-highlight")]
pub mod dummy_highlighter {}

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::WatLanguageService;

pub use lexer::token_type::WatTokenType;
pub use parser::element_type::WatElementType;
