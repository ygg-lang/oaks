#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! LLvm-ir support for the Oak language framework.

pub mod ast;
mod builder;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

pub use crate::{
    ast::LLirRoot,
    builder::LLirBuilder,
    language::LLvmLanguage,
    lexer::{LLvmLexer, token_type::LLvmTokenType},
    parser::LLirParser,
};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::LLirHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::LLirLanguageService;
pub use parser::element_type::LLvmElementType;
