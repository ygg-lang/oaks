#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Handlebars support for the Oak language framework.

pub mod ast;
pub mod builder;

mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

mod parser;

pub use crate::{
    ast::HandlebarsRoot,
    builder::HandlebarsBuilder,
    language::HandlebarsLanguage,
    lexer::{HandlebarsLexer, token_type::HandlebarsTokenType},
    parser::{HandlebarsParser, element_type::HandlebarsElementType},
};
