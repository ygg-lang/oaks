#![feature(new_range_api)]
#![warn(missing_docs)]
//! Mermaid support for the Oak language framework.

/// Abstract Syntax Tree (AST) definitions for Mermaid.
pub mod ast;
/// Incremental tree builder for Mermaid.
pub mod builder;
/// Language configuration for Mermaid.
pub mod language;
/// Lexer for Mermaid.
pub mod lexer;
/// Parser for Mermaid.
pub mod parser;

pub use crate::{
    ast::MermaidRoot,
    builder::MermaidBuilder,
    language::MermaidLanguage,
    lexer::{MermaidLexer, token_type::MermaidTokenType},
    parser::{MermaidParser, element_type::MermaidElementType},
};
