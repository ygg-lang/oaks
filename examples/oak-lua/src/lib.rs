#![feature(new_range_api)]
#![no_std]

extern crate alloc;

mod language;
mod lexer;
mod syntax;

pub use language::LuaLanguage;
pub use lexer::LuaLexer;
pub use syntax::LuaSyntaxKind;
