#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
mod formatter;
pub mod highlighter;
mod kind;
mod language;
mod lexer;
pub mod parser;

pub use crate::{
    ast::DartRoot,
    builder::DartBuilder,
    formatter::DartFormatter,
    highlighter::DartHighlighter,
    kind::{DartSyntaxKind, DartToken},
    language::DartLanguage,
    lexer::DartLexer,
};
