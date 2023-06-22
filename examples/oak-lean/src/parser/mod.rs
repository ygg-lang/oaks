use crate::{language::LeanLanguage, lexer::LeanLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, LeanLanguage, S>;

pub struct LeanParser<'config> {
    pub(crate) config: &'config LeanLanguage,
}

impl<'config> LeanParser<'config> {
    pub fn new(config: &'config LeanLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<LeanLanguage> for LeanParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LeanLanguage>) -> ParseOutput<'a, LeanLanguage> {
        let lexer = LeanLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
