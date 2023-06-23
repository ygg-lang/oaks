pub mod element_type;

use crate::{ast::PlantUmlRoot, lexer::PlantUmlLexer};
use core::range::Range;

pub struct PlantUmlParser<'a> {
    _lexer: PlantUmlLexer<'a>,
}

impl<'a> PlantUmlParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _lexer: PlantUmlLexer::new(input) }
    }

    pub fn parse(&mut self) -> PlantUmlRoot {
        PlantUmlRoot { elements: Vec::new(), span: Range { start: 0, end: 0 } }
    }
}
