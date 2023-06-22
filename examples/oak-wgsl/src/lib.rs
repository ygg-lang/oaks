#![feature(new_range_api)]
pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{ast::WgslRoot, kind::WgslSyntaxKind, language::WgslLanguage, lexer::WgslLexer, parser::WgslParser};
