#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

mod language;
mod lexer;
mod parser;
mod syntax;

pub use crate::{language::GsglLanguage, lexer::GsglLexer, parser::GsglParser, syntax::GsglSyntaxKind};
