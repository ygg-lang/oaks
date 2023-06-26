/// Element type definitions for D2.
pub mod element_type;

use crate::{D2TokenType, language::D2Language, lexer::D2Lexer};
use oak_core::{
    GreenNode, Parser,
    parser::{ParseCache, ParseOutput, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser for D2 diagram language.
pub struct D2Parser<'config> {
    config: &'config D2Language,
}

impl<'config> D2Parser<'config> {
    /// Creates a new D2Parser with the given language configuration.
    pub fn new(config: &'config D2Language) -> Self {
        Self { config }
    }
}

impl<'config> Parser<D2Language> for D2Parser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<D2Language>) -> ParseOutput<'a, D2Language> {
        let lexer = D2Lexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            while state.not_at_end() {
                if let Some(_) = self.parse_element(state) {
                    // Element was parsed and added to the sink
                }
                else {
                    // Skip unexpected tokens
                    state.advance();
                }
            }

            // The root node is automatically created by the parser framework
            Ok(state.sink.finish_node(0, element_type::D2ElementType::Root))
        })
    }
}

impl<'config> D2Parser<'config> {
    fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, D2Language, S>) -> Option<&'a GreenNode<'a, D2Language>> {
        // Try to parse a shape
        if let Some(shape) = self.parse_shape(state) {
            return Some(shape);
        }

        // Try to parse a connection
        if let Some(connection) = self.parse_connection(state) {
            return Some(connection);
        }

        None
    }

    fn parse_shape<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, D2Language, S>) -> Option<&'a GreenNode<'a, D2Language>> {
        // Check for identifier
        if !state.at(D2TokenType::Id) {
            return None;
        }

        let checkpoint = state.checkpoint();

        // Consume identifier
        state.bump();

        // Check for colon
        if state.at(D2TokenType::Colon) {
            state.bump();

            // Check for label
            if state.at(D2TokenType::Label) {
                state.bump();
            }
        }

        Some(state.finish_at(checkpoint, element_type::D2ElementType::Shape))
    }

    fn parse_connection<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, D2Language, S>) -> Option<&'a GreenNode<'a, D2Language>> {
        // Check for first identifier
        if !state.at(D2TokenType::Id) {
            return None;
        }

        let checkpoint = state.checkpoint();

        // Consume first identifier
        state.bump();

        // Check for arrow
        if !state.at(D2TokenType::Arrow) {
            return None;
        }

        // Consume arrow
        state.bump();

        // Check for second identifier
        if !state.at(D2TokenType::Id) {
            return None;
        }

        // Consume second identifier
        state.bump();

        Some(state.finish_at(checkpoint, element_type::D2ElementType::Connection))
    }
}
