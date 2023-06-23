#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! Kotlin support for the Oak language framework.

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

pub use ast::KotlinRoot;
pub use builder::KotlinBuilder;
pub use language::KotlinLanguage;
pub use lexer::{KotlinLexer, token_type::KotlinTokenType};
pub use parser::element_type::KotlinElementType;
