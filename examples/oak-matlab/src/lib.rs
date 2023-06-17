#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

pub use kind::MatlabSyntaxKind;
pub use language::MatlabLanguage;
pub use lexer::MatlabLexer;
