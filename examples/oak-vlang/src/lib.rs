#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! VLang support for the Oak language framework.

extern crate alloc;

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

pub use crate::{language::VLangLanguage, lexer::VLangLexer, parser::VLangParser};
pub use oak_core::{ElementType, TokenType};

// #[cfg(feature = "oak-highlight")]
// pub use crate::lsp::highlighter::VLangHighlighter;

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::VLangLanguageService;
// #[cfg(feature = "lsp")]
// pub use crate::lsp::formatter::VLangFormatter;
pub use lexer::token_type::VLangTokenType;
pub use parser::element_type::VLangElementType;
