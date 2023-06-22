#![feature(new_range_api)]

pub mod ast;
pub mod formatter;
pub mod kind;
pub mod language;
pub mod lexer;
#[cfg(feature = "lsp")]
pub mod lsp;
#[cfg(feature = "lsp")]
pub mod mcp;
pub mod parser;
#[cfg(feature = "lsp")]
pub mod semantic_tokens;

pub use crate::{kind::RbqSyntaxKind, language::RbqLanguage, lexer::RbqLexer, parser::RbqParser};
