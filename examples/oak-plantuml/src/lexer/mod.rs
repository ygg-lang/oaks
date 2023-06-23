pub mod token_type;

use crate::lexer::token_type::PlantUmlTokenType;
use core::range::Range;
use oak_core::Token;

pub type PlantUmlToken = Token<PlantUmlTokenType>;

pub struct PlantUmlLexer<'a> {
    _input: &'a str,
}

impl<'a> PlantUmlLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _input: input }
    }

    pub fn next_token(&mut self) -> PlantUmlToken {
        PlantUmlToken { kind: PlantUmlTokenType::Error, span: Range { start: 0, end: 0 } }
    }
}
