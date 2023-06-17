#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

// Re-exports
pub use kind::CssSyntaxKind;
pub use language::CssLanguage;
pub use lexer::CssLexer;
