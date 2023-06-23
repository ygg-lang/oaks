#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! Wit-component support for the Oak language framework.

pub mod ast;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use crate::{ast::WitRoot, language::WitLanguage, lexer::WitLexer, parser::WitParser};
pub use lexer::token_type::WitTokenType;
pub use parser::element_type::WitElementType;
