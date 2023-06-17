#![no_std]

extern crate alloc;

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

pub use crate::{
    ast::CppRoot,
    kind::CppSyntaxKind,
    language::CppLanguage,
    lexer::{CLexer, CppLexer},
};
