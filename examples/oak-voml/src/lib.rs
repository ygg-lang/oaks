#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Voml support for the Oak language framework.

extern crate alloc;

/// AST module.
pub mod ast;

/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser module.
pub mod parser;
pub use crate::{language::VomlLanguage, lexer::VomlLexer, parser::VomlParser};
/// Syntax module.
pub use oak_core::{ElementType, TokenType};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::VomlLanguageService;
pub use lexer::token_type::VomlTokenType;
pub use parser::element_type::VomlElementType;
