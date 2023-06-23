#![feature(new_range_api)]
#![warn(missing_docs)]
//! PlantUML support for the Oak language framework.

pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{
    ast::PlantUmlRoot,
    language::PlantUmlLanguage,
    lexer::{PlantUmlLexer, token_type::PlantUmlTokenType},
    parser::{PlantUmlParser, element_type::PlantUmlElementType},
};
