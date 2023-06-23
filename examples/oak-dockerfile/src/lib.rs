#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Dockerfile support for the Oak language framework.

pub mod ast;
mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
mod parser;

pub use crate::{
    ast::DockerfileRoot,
    language::DockerfileLanguage,
    lexer::{DockerfileLexer, token_type::DockerfileTokenType},
    parser::{DockerfileParser, element_type::DockerfileElementType},
};
