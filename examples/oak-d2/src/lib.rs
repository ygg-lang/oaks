#![feature(new_range_api)]
#![warn(missing_docs)]
//! D2 support for the Oak language framework.

/// Abstract Syntax Tree (AST) definitions for D2.
pub mod ast;
/// Builder for the D2 AST.
pub mod builder;
/// Language configuration for D2.
pub mod language;
/// Lexer implementation for D2.
pub mod lexer;
/// Parser implementation for D2.
pub mod parser;

pub use crate::{
    ast::D2Root,
    builder::D2Builder,
    language::D2Language,
    lexer::{D2Lexer, token_type::D2TokenType},
    parser::{D2Parser, element_type::D2ElementType},
};
