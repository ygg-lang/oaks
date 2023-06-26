#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Language module.
mod language;
/// Lexer module.
mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser module.
pub mod parser;

pub use crate::{
    ast::FortranRoot,
    builder::FortranBuilder,
    language::FortranLanguage,
    lexer::{FortranLexer, token_type::FortranTokenType},
    parser::element_type::FortranElementType,
};
