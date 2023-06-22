use crate::{language::ValaLanguage, lexer::ValaLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, ValaLanguage, S>;

pub struct ValaParser<'config> {
    pub(crate) _config: &'config ValaLanguage,
}

impl<'config> ValaParser<'config> {
    pub fn new(config: &'config ValaLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<ValaLanguage> for ValaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ValaLanguage>) -> ParseOutput<'a, ValaLanguage> {
        let lexer = ValaLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
