/// Element type definitions.
pub mod element_type;

use crate::{
    language::PrologLanguage,
    lexer::{PrologLexer, token_type::PrologTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::{Source, TextEdit},
};

/// Prolog parser.
pub struct PrologParser {
    language: PrologLanguage,
}

impl PrologParser {
    /// Creates a new `PrologParser` with the given language configuration.
    pub fn new(language: PrologLanguage) -> Self {
        Self { language }
    }

    fn parse_directive<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PrologTokenType::ColonMinus).ok();
        while !state.at(PrologTokenType::Dot) && state.not_at_end() {
            state.bump()
        }
        state.expect(PrologTokenType::Dot).ok();
        state.finish_at(checkpoint, crate::parser::element_type::PrologElementType::Directive);
    }

    fn parse_query<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PrologTokenType::QuestionMinus).ok();
        while !state.at(PrologTokenType::Dot) && state.not_at_end() {
            state.bump()
        }
        state.expect(PrologTokenType::Dot).ok();
        state.finish_at(checkpoint, crate::parser::element_type::PrologElementType::Query);
    }

    fn parse_clause<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        while !state.at(PrologTokenType::Dot) && state.not_at_end() {
            state.bump()
        }
        state.expect(PrologTokenType::Dot).ok();
        state.finish_at(checkpoint, crate::parser::element_type::PrologElementType::Clause);
    }
}

impl Parser<PrologLanguage> for PrologParser {
    fn parse<'s, S: Source + ?Sized>(&self, text: &'s S, edits: &[TextEdit], cache: &'s mut impl ParseCache<PrologLanguage>) -> ParseOutput<'s, PrologLanguage> {
        let lexer = PrologLexer::new(&self.language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if state.at(PrologTokenType::ColonMinus) {
                    self.parse_directive(state)
                }
                else if state.at(PrologTokenType::QuestionMinus) {
                    self.parse_query(state)
                }
                else {
                    self.parse_clause(state)
                }
            }
            Ok(state.finish_at(checkpoint, crate::parser::element_type::PrologElementType::Root))
        })
    }
}
