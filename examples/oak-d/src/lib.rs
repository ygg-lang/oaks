#![no_std]

extern crate alloc;

pub mod kind;
pub mod language;
pub mod lexer;

// 重新导出主要类型
pub use kind::DSyntaxKind;
pub use language::DLanguage;
pub use lexer::DLexer;
