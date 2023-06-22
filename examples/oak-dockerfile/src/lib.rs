#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod kind;
mod language;
mod lexer;
mod parser;

pub use crate::{
    ast::DockerfileRoot,
    kind::{DockerfileSyntaxKind, DockerfileToken},
    language::DockerfileLanguage,
    lexer::DockerfileLexer,
    parser::DockerfileParser,
};
