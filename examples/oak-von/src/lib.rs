#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

//! Von support for the Oak language framework.

extern crate alloc;

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
// pub mod formatter;
//
// pub mod highlighter;
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

pub use crate::{ast::VonValue, builder::VonBuilder, language::VonLanguage, lexer::VonLexer, parser::VonParser};

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::VonLanguageService;
pub use lexer::token_type::VonTokenType;
pub use parser::element_type::VonElementType;
