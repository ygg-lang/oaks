#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod kind;
mod language;
mod lexer;
pub mod parser;

mod builder;
mod formatter;
pub mod highlighter;

pub use crate::{
    ast::DHallRoot,
    builder::DHallBuilder,
    formatter::DHallFormatter,
    highlighter::DHallHighlighter,
    kind::{DHallSyntaxKind, DHallToken},
    language::DHallLanguage,
    lexer::DHallLexer,
};
