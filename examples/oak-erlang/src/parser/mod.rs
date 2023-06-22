use crate::{language::ErlangLanguage, lexer::ErlangLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, ErlangLanguage, S>;

pub struct ErlangParser<'config> {
    pub(crate) _config: &'config ErlangLanguage,
}

impl<'config> ErlangParser<'config> {
    pub fn new(config: &'config ErlangLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<ErlangLanguage> for ErlangParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ErlangLanguage>) -> ParseOutput<'a, ErlangLanguage> {
        let lexer = ErlangLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
