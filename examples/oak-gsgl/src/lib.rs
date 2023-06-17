#![feature(new_range_api)]
#![no_std]

extern crate alloc;

mod language;
mod lexer;
mod syntax;

pub use language::GsglLanguage;
pub use lexer::GsglLexer;
pub use syntax::GsglSyntaxKind;
