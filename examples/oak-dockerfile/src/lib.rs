#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// Root node of the Dockerfile AST.
pub mod ast;
/// Builder for the Dockerfile AST.
pub mod builder;
/// Language configuration for Dockerfile.
pub mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
mod parser;

pub use crate::{
    ast::DockerfileRoot,
    builder::DockerfileBuilder,
    language::DockerfileLanguage,
    lexer::{DockerfileLexer, token_type::DockerfileTokenType},
    parser::{DockerfileParser, element_type::DockerfileElementType},
};
