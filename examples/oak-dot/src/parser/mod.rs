use crate::{language::DotLanguage, lexer::DotLexer};
use oak_core::{
    parser::{ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DotLanguage, S>;

pub struct DotParser<'config> {
    pub(crate) _config: &'config DotLanguage,
}

impl<'config> DotParser<'config> {
    pub fn new(config: &'config DotLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DotLanguage> for DotParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DotLanguage>) -> ParseOutput<'a, DotLanguage> {
        let lexer = DotLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
