/// Element type definitions.
pub mod element_type;
pub use element_type::CoqElementType;

use crate::{
    language::CoqLanguage,
    lexer::{CoqLexer, token_type::CoqTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CoqLanguage, S>;

/// Coq parser implementation.
pub struct CoqParser<'config> {
    #[allow(dead_code)]
    pub(crate) config: &'config CoqLanguage,
}

impl<'config> CoqParser<'config> {
    /// Creates a new Coq parser with the given language configuration.
    pub fn new(config: &'config CoqLanguage) -> Self {
        Self { config }
    }

    /// Parses a declaration (Theorem, Lemma, Definition).
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Theorem/Lemma/Definition
        state.expect(CoqTokenType::Identifier)?;

        if state.at(CoqTokenType::Colon) {
            state.expect(CoqTokenType::Colon)?;
            self.parse_term(state)?
        }

        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        state.finish_at(checkpoint, CoqElementType::Declaration);
        Ok(())
    }

    /// Parses a proof block.
    fn parse_proof<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CoqTokenType::Proof)?;
        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        while state.not_at_end() && !state.at(CoqTokenType::Qed) && !state.at(CoqTokenType::Admitted) {
            state.bump()
        }

        if state.at(CoqTokenType::Qed) {
            state.expect(CoqTokenType::Qed)?
        }
        else {
            state.expect(CoqTokenType::Admitted)?
        }

        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        state.finish_at(checkpoint, CoqElementType::Statement);
        Ok(())
    }

    /// Parses a term.
    fn parse_term<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(CoqTokenType::Dot) && !state.at(CoqTokenType::Semicolon) {
            state.bump()
        }
        state.finish_at(checkpoint, CoqElementType::Expression);
        Ok(())
    }
}

impl<'config> Parser<CoqLanguage> for CoqParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CoqLanguage>) -> ParseOutput<'a, CoqLanguage> {
        let lexer = CoqLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() && !state.at(CoqTokenType::Eof) {
                if state.at(CoqTokenType::Theorem) || state.at(CoqTokenType::Lemma) || state.at(CoqTokenType::Definition) {
                    self.parse_declaration(state)?
                }
                else if state.at(CoqTokenType::Proof) {
                    self.parse_proof(state)?
                }
                else {
                    state.bump()
                }
            }

            Ok(state.finish_at(checkpoint, CoqElementType::Root))
        })
    }
}
