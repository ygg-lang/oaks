#![feature(new_range_api)]
pub mod ast;
mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

pub use crate::{ast::LlirRoot, builder::LlirBuilder, highlighter::LlirHighlighter, kind::LLvmSyntaxKind, language::LLvmLanguage, lexer::LlvmLexer, lsp::LlirLanguageService, parser::LlirParser};
