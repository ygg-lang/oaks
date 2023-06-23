#![feature(new_range_api)]
#![warn(missing_docs)]
//! Mermaid support for the Oak language framework.

pub mod ast;
pub mod builder;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{
    ast::MermaidRoot,
    builder::MermaidBuilder,
    language::MermaidLanguage,
    lexer::{MermaidLexer, token_type::MermaidTokenType},
    parser::{MermaidParser, element_type::MermaidElementType},
};
