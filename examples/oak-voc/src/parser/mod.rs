use crate::{language::VocLanguage, lexer::VocLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, VocLanguage, S>;

pub struct VocParser<'config> {
    pub(crate) config: &'config VocLanguage,
}

impl<'config> VocParser<'config> {
    pub fn new(config: &'config VocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VocLanguage> for VocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VocLanguage>) -> oak_core::ParseOutput<'a, VocLanguage> {
        let lexer = VocLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}
