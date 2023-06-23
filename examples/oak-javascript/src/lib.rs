#![doc = include_str!("readme.md")]
#![warn(missing_docs)]
#![feature(portable_simd)]
#![feature(new_range_api)]
//! Javascript support for the Oak language framework.

#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;

pub use lexer::token_type::JavaScriptTokenType;
pub use parser::element_type::JavaScriptElementType;
