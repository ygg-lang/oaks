#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

// 重新导出主要类型
pub use crate::{
    kind::{RustSyntaxKind, RustToken},
    language::RustLanguage,
    lexer::RustLexer,
    parser::RustParser,
};
