pub mod element_type;

use crate::{
    language::StylusLanguage,
    lexer::{StylusLexer, token_type::StylusTokenType},
    parser::element_type::StylusElementType,
};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, StylusLanguage, S>;

/// Stylus parser implementation.
pub struct StylusParser<'config> {
    pub(crate) config: &'config StylusLanguage,
}

impl<'config> StylusParser<'config> {
    /// Create a new stylus parser.
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }

    /// Parse a node.
    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(StylusTokenType::Identifier) | Some(StylusTokenType::Dot) | Some(StylusTokenType::Hash) | Some(StylusTokenType::Ampersand) | Some(StylusTokenType::Star) => {
                if self.is_declaration(state) {
                    self.parse_declaration(state)
                }
                else {
                    self.parse_rule(state)
                }
            }
            Some(StylusTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, StylusElementType::Comment);
                Ok(())
            }
            _ => {
                state.advance();
                Ok(())
            }
        }
    }

    /// Check if the current token starts a declaration.
    fn is_declaration<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> bool {
        match state.tokens.peek_at(1) {
            Some(token) => matches!(token.kind, StylusTokenType::Colon | StylusTokenType::Whitespace),
            None => false,
        }
    }

    /// Parse a declaration.
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        let prop_cp = state.checkpoint();
        state.expect(StylusTokenType::Identifier)?;
        state.finish_at(prop_cp, StylusElementType::Property);

        if state.at(StylusTokenType::Colon) {
            state.bump();
        }

        let val_cp = state.checkpoint();
        while state.not_at_end() && !state.at(StylusTokenType::Newline) && !state.at(StylusTokenType::Semicolon) && !state.at(StylusTokenType::RightBrace) {
            state.advance();
        }
        state.finish_at(val_cp, StylusElementType::Value);

        state.eat(StylusTokenType::Semicolon);
        state.finish_at(cp, StylusElementType::Rule); // Representing a declaration as a simple rule for now
        Ok(())
    }

    /// Parse a rule.
    fn parse_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Parse selector
        let sel_cp = state.checkpoint();
        while state.not_at_end() && !state.at(StylusTokenType::LeftBrace) && !state.at(StylusTokenType::Newline) {
            state.advance();
        }
        state.finish_at(sel_cp, StylusElementType::Selector);

        if state.at(StylusTokenType::LeftBrace) {
            self.parse_block(state)?;
        }
        else if state.at(StylusTokenType::Newline) {
            state.bump();
            // In a real Stylus parser, we'd handle indentation here.
            // For now, we just continue parsing nodes.
        }

        state.finish_at(cp, StylusElementType::Rule);
        Ok(())
    }

    /// Parse a block.
    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(StylusTokenType::LeftBrace)?;
        while state.not_at_end() && !state.at(StylusTokenType::RightBrace) {
            self.parse_node(state)?;
        }
        state.expect(StylusTokenType::RightBrace)?;
        state.finish_at(cp, StylusElementType::Block);
        Ok(())
    }
}

impl<'config> Parser<StylusLanguage> for StylusParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<StylusLanguage>) -> ParseOutput<'a, StylusLanguage> {
        let lexer = StylusLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(checkpoint, StylusElementType::Root))
        })
    }
}
