use crate::{language::TclLanguage, lexer::TclLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, TclLanguage, S>;

pub struct TclParser<'config> {
    pub(crate) config: &'config TclLanguage,
}

impl<'config> TclParser<'config> {
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TclLanguage> for TclParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TclLanguage>) -> ParseOutput<'a, TclLanguage> {
        let lexer = TclLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
