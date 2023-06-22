use crate::language::PerlLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, PerlLanguage, S>;

pub struct PerlParser<'config> {
    pub(crate) config: &'config PerlLanguage,
}

impl<'config> PerlParser<'config> {
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<PerlLanguage> for PerlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PerlLanguage>) -> ParseOutput<'a, PerlLanguage> {
        let lexer = crate::lexer::PerlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
