/// Element types and categories for the PlantUML language.
pub mod element_type;

use crate::{ast::PlantUmlRoot, lexer::PlantUmlLexer};
use core::range::Range;

/// A parser for the PlantUML language.
pub struct PlantUmlParser<'a> {
    _lexer: PlantUmlLexer<'a>,
}

impl<'a> PlantUmlParser<'a> {
    /// Create a new parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Self { _lexer: PlantUmlLexer::new(input) }
    }

    /// Parse the input and return a root node.
    pub fn parse(&mut self) -> PlantUmlRoot {
        PlantUmlRoot { elements: Vec::new(), span: Range { start: 0, end: 0 } }
    }
}
