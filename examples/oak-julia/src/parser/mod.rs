use crate::{language::JuliaLanguage, lexer::JuliaLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, JuliaLanguage, S>;

pub struct JuliaParser<'config> {
    pub(crate) config: &'config JuliaLanguage,
}

impl<'config> JuliaParser<'config> {
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<JuliaLanguage> for JuliaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JuliaLanguage>) -> ParseOutput<'a, JuliaLanguage> {
        let lexer = JuliaLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
