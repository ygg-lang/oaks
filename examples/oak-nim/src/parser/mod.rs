use crate::{language::NimLanguage, lexer::NimLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, NimLanguage, S>;

pub struct NimParser<'config> {
    pub(crate) config: &'config NimLanguage,
}

impl<'config> NimParser<'config> {
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<NimLanguage> for NimParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<NimLanguage>) -> ParseOutput<'a, NimLanguage> {
        let lexer = NimLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
