#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod syntax;

// 重新导出主要类型
pub use crate::{kind::ScalaSyntaxKind, language::ScalaLanguage};
