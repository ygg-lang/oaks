pub mod element_type;

use crate::{language::DjangoLanguage, lexer::DjangoLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, DjangoLanguage, S>;

pub struct DjangoParser<'config> {
    pub(crate) _config: &'config DjangoLanguage,
}

impl<'config> DjangoParser<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DjangoLanguage> for DjangoParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DjangoLanguage>) -> oak_core::ParseOutput<'a, DjangoLanguage> {
        let lexer = DjangoLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
