#![feature(new_range_api)]
#![no_std]

pub mod kind;
pub mod language;
pub mod lexer;

pub use crate::{kind::AdaSyntaxKind, language::AdaLanguage, lexer::AdaLexer};
