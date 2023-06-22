#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
#[cfg(feature = "oak-highlight")]
mod highlighter;
mod kind;
mod language;
mod lexer;
mod parser;

#[cfg(feature = "oak-highlight")]
pub use crate::highlighter::DjangoHighlighter;
pub use crate::{
    ast::DjangoRoot,
    kind::{DjangoSyntaxKind, DjangoToken},
    language::DjangoLanguage,
    lexer::DjangoLexer,
    parser::DjangoParser,
};
