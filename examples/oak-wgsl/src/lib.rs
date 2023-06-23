#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Wgsl support for the Oak language framework.

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

pub use crate::{ast::WgslRoot, language::WgslLanguage, lexer::WgslLexer, parser::WgslParser};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::WgslLanguageService;
pub use lexer::token_type::WgslTokenType;
pub use parser::element_type::WgslElementType;
