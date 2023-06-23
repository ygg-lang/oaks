pub mod element_type;

use crate::{language::SchemeLanguage, lexer::SchemeLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, SchemeLanguage, S>;

pub struct SchemeParser<'config> {
    pub(crate) config: &'config SchemeLanguage,
}

impl<'config> SchemeParser<'config> {
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<SchemeLanguage> for SchemeParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SchemeLanguage>) -> ParseOutput<'a, SchemeLanguage> {
        let lexer = SchemeLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
