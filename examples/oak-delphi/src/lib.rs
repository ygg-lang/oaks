#![no_std]

pub mod kind;
pub mod language;
pub mod lexer;

pub use kind::DelphiSyntaxKind;
pub use language::DelphiLanguage;
pub use lexer::DelphiLexer;
