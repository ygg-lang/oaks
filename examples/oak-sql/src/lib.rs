#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod highlighter;
pub mod language;
pub mod lexer;
// pub mod parser;  // 暂时注释掉，需要重新实现
pub mod syntax;

// 重新导出主要类型
pub use crate::{language::SqlLanguage, syntax::SqlSyntaxKind};
