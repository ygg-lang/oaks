#![feature(new_range_api)]
#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

// 重新导出主要类型
pub use crate::{
    kind::RustSyntaxKind,
    language::RustLanguage,
    lexer::{RustLexer, RustToken},
    parser::RustParser,
};
