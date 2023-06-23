pub mod token_type;

use crate::lexer::token_type::StructurizrTokenType;
use core::range::Range;
use oak_core::Token;

pub type StructurizrToken = Token<StructurizrTokenType>;

pub struct StructurizrLexer<'a> {
    _input: &'a str,
}

impl<'a> StructurizrLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _input: input }
    }

    pub fn next_token(&mut self) -> StructurizrToken {
        StructurizrToken { kind: StructurizrTokenType::Error, span: Range { start: 0, end: 0 } }
    }
}
