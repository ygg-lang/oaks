#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

// 统一导出主要类型
pub use crate::{kind::JasminSyntaxKind, language::JasminLanguage};
