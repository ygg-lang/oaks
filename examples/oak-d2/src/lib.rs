#![feature(new_range_api)]
#![warn(missing_docs)]
//! D2 support for the Oak language framework.

pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{
    ast::D2Root,
    language::D2Language,
    lexer::{D2Lexer, token_type::D2TokenType},
    parser::{D2Parser, element_type::D2ElementType},
};
