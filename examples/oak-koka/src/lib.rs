#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! Koka support for the Oak language framework.

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

pub use ast::KokaRoot;
pub use builder::KokaBuilder;
pub use language::KokaLanguage;
pub use lexer::{KokaLexer, token_type::KokaTokenType};
pub use parser::element_type::KokaElementType;
