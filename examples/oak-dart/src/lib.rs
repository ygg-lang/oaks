#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Dart support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(feature = "lsp")]
pub mod lsp;
/// Parser module.
pub mod parser;

pub use crate::{language::DartLanguage, lexer::DartLexer, parser::DartParser};

//
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DartHighlighter;

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::DartLanguageService;
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::DartFormatter;
pub use lexer::token_type::DartTokenType;
pub use parser::element_type::DartElementType;
