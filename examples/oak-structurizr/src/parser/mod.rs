pub mod element_type;

use crate::{ast::StructurizrRoot, lexer::StructurizrLexer};
use core::range::Range;

pub struct StructurizrParser<'a> {
    _lexer: StructurizrLexer<'a>,
}

impl<'a> StructurizrParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _lexer: StructurizrLexer::new(input) }
    }

    pub fn parse(&mut self) -> StructurizrRoot {
        StructurizrRoot { elements: Vec::new(), span: Range { start: 0, end: 0 } }
    }
}
