use crate::{language::DartLanguage, lexer::DartLexer};
use oak_core::{
    parser::{ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DartLanguage, S>;

pub struct DartParser<'config> {
    pub(crate) _config: &'config DartLanguage,
}

impl<'config> DartParser<'config> {
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DartLanguage> for DartParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DartLanguage>) -> ParseOutput<'a, DartLanguage> {
        let lexer = DartLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
