#![no_std]

pub mod kind;
pub mod language;
pub mod lexer;

pub use kind::DotSyntaxKind;
pub use language::DotLanguage;
pub use lexer::DotLexer;
