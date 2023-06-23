pub mod element_type;

use crate::{language::ElmLanguage, lexer::ElmLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, ElmLanguage, S>;

pub struct ElmParser<'config> {
    pub(crate) _config: &'config ElmLanguage,
}

impl<'config> Parser<ElmLanguage> for ElmParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ElmLanguage>) -> ParseOutput<'a, ElmLanguage> {
        let lexer = ElmLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'config> ElmParser<'config> {
    pub fn new(config: &'config ElmLanguage) -> Self {
        Self { _config: config }
    }
}
