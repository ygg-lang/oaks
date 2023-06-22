use crate::{kind::PrologSyntaxKind, language::PrologLanguage, lexer::PrologLexer};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::{Source, TextEdit},
};

pub struct PrologParser<'a> {
    language: &'a PrologLanguage,
}

impl<'a> PrologParser<'a> {
    pub fn new(language: &'a PrologLanguage) -> Self {
        Self { language }
    }

    fn parse_directive<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PrologSyntaxKind::ColonMinus).ok();
        while !state.at(PrologSyntaxKind::Dot) && state.not_at_end() {
            state.bump();
        }
        state.expect(PrologSyntaxKind::Dot).ok();
        state.finish_at(checkpoint, PrologSyntaxKind::Directive);
    }

    fn parse_query<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PrologSyntaxKind::QuestionMinus).ok();
        while !state.at(PrologSyntaxKind::Dot) && state.not_at_end() {
            state.bump();
        }
        state.expect(PrologSyntaxKind::Dot).ok();
        state.finish_at(checkpoint, PrologSyntaxKind::Query);
    }

    fn parse_clause<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) {
        let checkpoint = state.checkpoint();
        while !state.at(PrologSyntaxKind::Dot) && state.not_at_end() {
            state.bump();
        }
        state.expect(PrologSyntaxKind::Dot).ok();
        state.finish_at(checkpoint, PrologSyntaxKind::Clause);
    }

    fn parse_root_internal<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, PrologLanguage, S>) -> Result<&'s GreenNode<'s, PrologLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            if state.at(PrologSyntaxKind::ColonMinus) {
                self.parse_directive(state);
            }
            else if state.at(PrologSyntaxKind::QuestionMinus) {
                self.parse_query(state);
            }
            else {
                self.parse_clause(state);
            }
        }
        Ok(state.finish_at(checkpoint, PrologSyntaxKind::Root))
    }
}

impl<'a> Parser<PrologLanguage> for PrologParser<'a> {
    fn parse<'s, S: Source + ?Sized>(&self, text: &'s S, edits: &[TextEdit], cache: &'s mut impl ParseCache<PrologLanguage>) -> ParseOutput<'s, PrologLanguage> {
        let lexer = PrologLexer::new(self.language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
