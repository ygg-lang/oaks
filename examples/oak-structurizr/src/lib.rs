#![feature(new_range_api)]
#![warn(missing_docs)]
//! Structurizr support for the Oak language framework.

pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{
    ast::StructurizrRoot,
    language::StructurizrLanguage,
    lexer::{StructurizrLexer, token_type::StructurizrTokenType},
    parser::{StructurizrParser, element_type::StructurizrElementType},
};
