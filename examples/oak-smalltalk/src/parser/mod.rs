pub mod element_type;

use crate::{language::SmalltalkLanguage, lexer::SmalltalkLexer, parser::element_type::SmalltalkElementType};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, SmalltalkLanguage, S>;

pub struct SmalltalkParser<'config> {
    pub(crate) _config: &'config SmalltalkLanguage,
}

impl<'config> SmalltalkParser<'config> {
    pub fn new(config: &'config SmalltalkLanguage) -> Self {
        Self { _config: config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SmalltalkLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump()
        }

        let root = state.finish_at(checkpoint, crate::parser::element_type::SmalltalkElementType::Root);
        Ok(root)
    }
}

impl<'config> Parser<SmalltalkLanguage> for SmalltalkParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SmalltalkLanguage>) -> ParseOutput<'a, SmalltalkLanguage> {
        let lexer = SmalltalkLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
