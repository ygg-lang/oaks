/// Matlab element type definitions
pub mod element_type;

use crate::{
    language::MatlabLanguage,
    lexer::{MatlabLexer, token_type::MatlabTokenType},
    parser::element_type::MatlabElementType,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, MatlabLanguage, S>;

/// MATLAB parser implementation
pub struct MatlabParser<'a> {
    pub(crate) _language: &'a MatlabLanguage,
}

impl<'a> MatlabParser<'a> {
    /// Creates a new Matlab parser
    pub fn new(language: &'a MatlabLanguage) -> Self {
        Self { _language: language }
    }
}

impl<'p> Parser<MatlabLanguage> for MatlabParser<'p> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MatlabLanguage>) -> ParseOutput<'a, MatlabLanguage> {
        let lexer = MatlabLexer::new(self._language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                self.parse_statement(state)
            }

            Ok(state.finish_at(cp, MatlabElementType::Script))
        })
    }
}

impl<'p> MatlabParser<'p> {
    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();

        match state.peek_kind() {
            Some(MatlabTokenType::Function) => self.parse_function_def(state),
            Some(MatlabTokenType::If) => self.parse_if_statement(state),
            _ => {
                self.parse_expression(state);
                if state.at(MatlabTokenType::Semicolon) {
                    state.bump()
                }
            }
        }

        state.finish_at(checkpoint, MatlabElementType::Statement);
    }

    fn parse_function_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // function
        // ... simple function parsing logic
        state.finish_at(checkpoint, MatlabElementType::FunctionDef);
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        state.bump(); // if
        self.parse_expression(state);
        // ... simple if parsing logic
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump();
        state.finish_at(checkpoint, MatlabElementType::Expression);
    }
}
