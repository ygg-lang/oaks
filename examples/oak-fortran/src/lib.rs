#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

pub use self::{
    kind::{FortranSyntaxKind, FortranToken},
    language::FortranLanguage,
    lexer::FortranLexer,
};
