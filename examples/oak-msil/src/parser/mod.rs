pub mod element_type;

use crate::{language::MsilLanguage, lexer::MsilLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, MsilLanguage, S>;

pub struct MsilParser<'config> {
    pub(crate) _config: &'config MsilLanguage,
}

impl<'config> MsilParser<'config> {
    pub fn new(config: &'config MsilLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<MsilLanguage> for MsilParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MsilLanguage>) -> oak_core::ParseOutput<'a, MsilLanguage> {
        let lexer = MsilLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
