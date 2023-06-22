use crate::{language::TypstLanguage, lexer::TypstLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, TypstLanguage, S>;

pub struct TypstParser<'config> {
    pub(crate) config: &'config TypstLanguage,
}

impl<'config> TypstParser<'config> {
    pub fn new(config: &'config TypstLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TypstLanguage> for TypstParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TypstLanguage>) -> ParseOutput<'a, TypstLanguage> {
        let lexer = TypstLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
