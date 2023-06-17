#![feature(new_range_api)]
#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

// Re-exports
pub use kind::CoqSyntaxKind;
pub use language::CoqLanguage;
pub use lexer::CoqLexer;
