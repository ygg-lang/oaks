/// Token types and syntax kinds for PlantUML.
pub mod token_type;

use crate::lexer::token_type::PlantUmlTokenType;
use core::range::Range;
use oak_core::Token;

/// Represents a single token in PlantUML.
pub type PlantUmlToken = Token<PlantUmlTokenType>;

/// Lexer for the PlantUML language.
pub struct PlantUmlLexer<'a> {
    _input: &'a str,
}

impl<'a> PlantUmlLexer<'a> {
    /// Creates a new lexer for the given input string.
    pub fn new(input: &'a str) -> Self {
        Self { _input: input }
    }

    /// Returns the next token from the input.
    pub fn next_token(&mut self) -> PlantUmlToken {
        PlantUmlToken { kind: PlantUmlTokenType::Error, span: Range { start: 0, end: 0 } }
    }
}
