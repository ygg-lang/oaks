#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{kind::WolframSyntaxKind, language::WolframLanguage, lexer::WolframLexer, parser::WolframParser};
