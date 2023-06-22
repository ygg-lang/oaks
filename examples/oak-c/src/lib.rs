#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
mod language;
mod lexer;
mod parser;

// Re-exports
pub use crate::{
    ast::CRoot,
    builder::CBuilder,
    language::CLanguage,
    lexer::{CLexer, CTokenType},
    parser::{CElementType, CParser},
};
