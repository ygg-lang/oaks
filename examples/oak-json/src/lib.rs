#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
#[cfg(feature = "highlight")]
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;

// 重新导出主要类型
pub use crate::{kind::JsonSyntaxKind, language::JsonLanguage};
