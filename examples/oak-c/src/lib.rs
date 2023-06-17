#![feature(new_range_api)]
#![no_std]

extern crate alloc;

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

// Re-exports
pub use crate::{ast::CRoot, kind::CSyntaxKind, language::CLanguage, lexer::CLexer};
