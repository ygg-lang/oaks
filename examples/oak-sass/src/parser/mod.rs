/// Element types for the Sass language.
pub mod element_type;

use crate::{
    language::SassLanguage,
    lexer::{SassLexer, token_type::SassTokenType},
    parser::element_type::SassElementType,
};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, SassLanguage, S>;

/// Parser for the Sass language.
pub struct SassParser<'config> {
    pub(crate) config: &'config SassLanguage,
}

impl<'config> SassParser<'config> {
    /// Creates a new Sass parser with the given configuration.
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(SassTokenType::Variable) => self.parse_variable_declaration(state),
            Some(SassTokenType::Identifier) | Some(SassTokenType::Dot) | Some(SassTokenType::Hash) => self.parse_rule(state),
            Some(SassTokenType::LineComment) | Some(SassTokenType::BlockComment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, SassElementType::Comment);
                Ok(())
            }
            _ => {
                state.advance();
                Ok(())
            }
        }
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(SassTokenType::Variable)?;
        state.expect(SassTokenType::Colon)?;
        while state.not_at_end() && !state.at(SassTokenType::Semicolon) && !state.at(SassTokenType::Newline) {
            state.advance();
        }
        state.eat(SassTokenType::Semicolon);
        state.finish_at(cp, SassElementType::Variable);
        Ok(())
    }

    fn parse_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        // Parse selector
        while state.not_at_end() && !state.at(SassTokenType::LeftBrace) {
            state.advance();
        }

        if state.eat(SassTokenType::LeftBrace) {
            while state.not_at_end() && !state.at(SassTokenType::RightBrace) {
                self.parse_node(state)?;
            }
            state.expect(SassTokenType::RightBrace).ok();
        }

        state.finish_at(cp, SassElementType::Selector);
        Ok(())
    }
}

impl<'config> Parser<SassLanguage> for SassParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SassLanguage>) -> ParseOutput<'a, SassLanguage> {
        let lexer = SassLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_node(state)?;
            }

            Ok(state.finish_at(checkpoint, SassElementType::SourceFile))
        })
    }
}
