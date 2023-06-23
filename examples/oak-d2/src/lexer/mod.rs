pub mod token_type;

use crate::lexer::token_type::D2TokenType;
use core::range::Range;
use oak_core::Token;

pub type D2Token = Token<D2TokenType>;

pub struct D2Lexer<'a> {
    _input: &'a str,
}

impl<'a> D2Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { _input: input }
    }

    pub fn next_token(&mut self) -> D2Token {
        D2Token { kind: D2TokenType::Error, span: Range { start: 0, end: 0 } }
    }
}
