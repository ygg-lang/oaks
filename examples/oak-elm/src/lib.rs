#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module for Elm.
pub mod ast;
/// Builder module for Elm.
pub mod builder;
/// Language configuration for Elm.
pub mod language;
/// Lexer for Elm.
pub mod lexer;
/// LSP support for Elm.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser for Elm.
pub mod parser;

pub use crate::{
    ast::ElmRoot,
    builder::ElmBuilder,
    language::ElmLanguage,
    lexer::{ElmLexer, token_type::ElmTokenType},
    parser::element_type::ElmElementType,
};
