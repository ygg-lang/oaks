#![feature(new_range_api)]
#![warn(missing_docs)]
//! PlantUML support for the Oak language framework.

/// Abstract Syntax Tree (AST) for PlantUML.
pub mod ast;
/// Language configuration for PlantUML.
pub mod language;
/// Lexer for PlantUML tokenization.
pub mod lexer;
/// Parser for PlantUML syntax analysis.
pub mod parser;

pub use crate::{
    ast::PlantUmlRoot,
    language::PlantUmlLanguage,
    lexer::{PlantUmlLexer, token_type::PlantUmlTokenType},
    parser::{PlantUmlParser, element_type::PlantUmlElementType},
};
