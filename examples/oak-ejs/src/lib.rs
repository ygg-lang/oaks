#![doc = include_str!("readme.md")]
#![warn(missing_docs)]
#![feature(portable_simd)]
#![feature(new_range_api)]

#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// AST module for EJS templates
pub mod ast;
/// Language configuration for EJS templates
pub mod language;
/// Lexer module for EJS templates
pub mod lexer;
/// Parser module for EJS templates
pub mod parser;

pub use lexer::token_type::EjsTokenType;
pub use parser::element_type::EjsElementType;
