/// Element types and categories for the Structurizr language.
pub mod element_type;

use crate::{ast::StructurizrRoot, lexer::StructurizrLexer};
use core::range::Range;

/// A parser for the Structurizr DSL.
pub struct StructurizrParser<'a> {
    _lexer: StructurizrLexer<'a>,
}

impl<'a> StructurizrParser<'a> {
    /// Create a new parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Self { _lexer: StructurizrLexer::new(input) }
    }

    /// Parse the input and return a root node.
    pub fn parse(&mut self) -> StructurizrRoot {
        StructurizrRoot { elements: Vec::new(), span: Range { start: 0, end: 0 }, ..Default::default() }
    }
}
