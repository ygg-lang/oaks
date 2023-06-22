use crate::{language::ElixirLanguage, lexer::ElixirLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, ElixirLanguage, S>;

pub struct ElixirParser<'config> {
    pub(crate) _config: &'config ElixirLanguage,
}

impl<'config> Parser<ElixirLanguage> for ElixirParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ElixirLanguage>) -> ParseOutput<'a, ElixirLanguage> {
        let lexer = ElixirLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'config> ElixirParser<'config> {
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { _config: config }
    }
}
