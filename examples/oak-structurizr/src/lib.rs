#![feature(new_range_api)]
#![warn(missing_docs)]
//! Structurizr support for the Oak language framework.

/// Abstract Syntax Tree (AST) definitions for Structurizr.
pub mod ast;
/// Language configuration for Structurizr.
pub mod language;
/// Lexer for Structurizr.
pub mod lexer;
/// Parser for Structurizr.
pub mod parser;

pub use crate::{
    ast::StructurizrRoot,
    language::StructurizrLanguage,
    lexer::StructurizrLexer,
    parser::{StructurizrParser, element_type::StructurizrElementType},
};

pub use lexer::token_type::StructurizrTokenType;
