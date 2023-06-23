#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Django support for the Oak language framework.

pub mod ast;
mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
mod parser;

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DjangoHighlighter;
pub use crate::{
    ast::DjangoRoot,
    language::DjangoLanguage,
    lexer::{DjangoLexer, token_type::DjangoTokenType},
    parser::{DjangoParser, element_type::DjangoElementType},
};
