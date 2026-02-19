#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// Erlang AST module.
pub mod ast;
/// Erlang builder module.
pub mod builder;
/// Erlang language configuration.
pub mod language;
/// Erlang lexer module.
pub mod lexer;
/// Erlang LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Erlang parser module.
pub mod parser;

pub use crate::{
    ast::ErlangRoot,
    builder::ErlangBuilder,
    language::ErlangLanguage,
    lexer::{ErlangLexer, token_type::ErlangTokenType},
    parser::element_type::ErlangElementType,
};
