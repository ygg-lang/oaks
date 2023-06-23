pub mod element_type;

use crate::{language::PurescriptLanguage, lexer::PurescriptLexer};
use oak_core::{
    TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, PurescriptLanguage, S>;

pub struct PurescriptParser<'config> {
    pub(crate) _config: &'config PurescriptLanguage,
}

impl<'config> PurescriptParser<'config> {
    pub fn new(config: &'config PurescriptLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<PurescriptLanguage> for PurescriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<PurescriptLanguage>) -> oak_core::ParseOutput<'a, PurescriptLanguage> {
        let lexer = PurescriptLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
