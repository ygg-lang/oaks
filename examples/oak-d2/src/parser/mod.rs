pub mod element_type;

use crate::{ast::D2Root, lexer::D2Lexer};
use core::range::Range;

pub struct D2Parser<'a> {
    _lexer: D2Lexer<'a>,
}

impl<'a> D2Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _lexer: D2Lexer::new(input) }
    }

    pub fn parse(&mut self) -> D2Root {
        D2Root { elements: Vec::new(), span: Range { start: 0, end: 0 } }
    }
}
