use crate::{language::RLanguage, lexer::RLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, RLanguage, S>;

pub struct RParser<'config> {
    pub(crate) config: &'config RLanguage,
}

impl<'config> RParser<'config> {
    pub fn new(config: &'config RLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<RLanguage> for RParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RLanguage>) -> ParseOutput<'a, RLanguage> {
        let lexer = RLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
