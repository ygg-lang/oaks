#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Ascii-doc support for the Oak language framework.

/// AST definitions for AsciiDoc.
pub mod ast;
/// Builder for the AsciiDoc AST.
pub mod builder;
mod language;
/// Lexer for AsciiDoc.
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
/// LSP and IDE support for AsciiDoc.
pub mod lsp;
/// Parser for AsciiDoc.
pub mod parser;

pub use crate::{
    ast::AsciiDocRoot,
    builder::AsciiDocBuilder,
    language::AsciiDocLanguage,
    lexer::{AsciiDocLexer, token_type::AsciiDocTokenType},
    parser::AsciiDocParser,
};
pub use parser::element_type::AsciiDocElementType;
