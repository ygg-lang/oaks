use crate::language::DelphiLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DelphiLanguage, S>;

pub struct DelphiParser<'config> {
    pub(crate) _config: &'config DelphiLanguage,
}

impl<'config> DelphiParser<'config> {
    pub fn new(config: &'config DelphiLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DelphiLanguage> for DelphiParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DelphiLanguage>) -> ParseOutput<'a, DelphiLanguage> {
        let lexer = crate::lexer::DelphiLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
