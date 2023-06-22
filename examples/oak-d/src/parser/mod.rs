use crate::{language::DLanguage, lexer::DLexer};
use oak_core::{
    TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DLanguage, S>;

pub struct DParser<'config> {
    pub(crate) _config: &'config DLanguage,
}

impl<'config> DParser<'config> {
    pub fn new(config: &'config DLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DLanguage> for DParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DLanguage>) -> oak_core::ParseOutput<'a, DLanguage> {
        let lexer = DLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
