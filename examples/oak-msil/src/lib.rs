#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod errors;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;
pub mod syntax;

pub use crate::{kind::MsilToken, language::MsilLanguage, lexer::MsilLexer, parser::MsilParser, syntax::MsilSyntaxKind};
