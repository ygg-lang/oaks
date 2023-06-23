#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![allow(missing_docs)]
//! Typescript support for the Oak language framework.

pub mod ast;
pub mod builder;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

// Re-exports
pub use crate::{
    ast::TypeScriptRoot,
    builder::TypeScriptBuilder,
    language::TypeScriptLanguage,
    lexer::{TypeScriptLexer, token_type::TypeScriptTokenType},
    parser::{TypeScriptParser, element_type::TypeScriptElementType},
};

#[cfg(feature = "lsp")]
pub use crate::lsp::{TypeScriptLanguageService, formatter::TypeScriptFormatter, highlighter::TypeScriptHighlighter};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_typescript_mcp;
