#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

// Re-exports
pub use kind::CsvSyntaxKind;
pub use language::CsvLanguage;
pub use lexer::CsvLexer;
