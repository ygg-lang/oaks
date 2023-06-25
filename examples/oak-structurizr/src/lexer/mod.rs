/// Token types for the Structurizr language.
pub mod token_type;

use crate::lexer::token_type::StructurizrTokenType;
use core::range::Range;
use oak_core::Token;

/// A token in the Structurizr language.
pub type StructurizrToken = Token<StructurizrTokenType>;

/// A lexer for the Structurizr language.
pub struct StructurizrLexer<'a> {
    _input: &'a str,
}

impl<'a> StructurizrLexer<'a> {
    /// Creates a new Structurizr lexer with the given input.
    pub fn new(input: &'a str) -> Self {
        Self { _input: input }
    }

    /// Returns the next token from the input.
    pub fn next_token(&mut self) -> StructurizrToken {
        StructurizrToken { kind: StructurizrTokenType::Error, span: Range { start: 0, end: 0 } }
    }
}
