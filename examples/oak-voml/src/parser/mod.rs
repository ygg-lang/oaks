pub mod element_type;

use crate::{language::VomlLanguage, lexer::VomlLexer};
use oak_core::{
    Source, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, VomlLanguage, S>;

pub struct VomlParser<'config> {
    pub(crate) config: &'config VomlLanguage,
}

impl<'config> VomlParser<'config> {
    pub fn new(config: &'config VomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VomlLanguage> for VomlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VomlLanguage>) -> ParseOutput<'a, VomlLanguage> {
        let lexer = VomlLexer::new(self.config);
        parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}
