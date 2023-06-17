#![no_std]
#![feature(new_range_api)]

extern crate alloc;

// Julia language support
pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

// 重新导出主要类型
pub use crate::{
    ast::{ExprKind, JuliaAst, ModuleNode, StmtKind},
    kind::{JuliaSyntaxKind, JuliaToken},
    language::JuliaLanguage,
    lexer::JuliaLexer,
};
