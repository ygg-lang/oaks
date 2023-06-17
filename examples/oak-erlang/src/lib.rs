#![no_std]

pub mod kind;
pub mod language;
pub mod lexer;

pub use kind::ErlangSyntaxKind;
pub use language::ErlangLanguage;
pub use lexer::ErlangLexer;
