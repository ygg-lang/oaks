#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Voc support for the Oak language framework.

extern crate alloc;

/// AST module.
pub mod ast;

/// Kind definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser module.
pub mod parser;

pub use crate::{language::VocLanguage, lexer::VocLexer, parser::VocParser};
pub use oak_core::{ElementType, TokenType};

// #[cfg(feature = "oak-highlight")]
// pub use crate::lsp::highlighter::VocHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::VocLanguageService;
// #[cfg(feature = "lsp")]
// pub use crate::lsp::formatter::VocFormatter;
pub use lexer::token_type::VocTokenType;
pub use parser::element_type::VocElementType;
