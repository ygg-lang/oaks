use crate::{language::SolidityLanguage, lexer::SolidityLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, SolidityLanguage, S>;

pub struct SolidityParser<'config> {
    pub(crate) _config: &'config SolidityLanguage,
}

impl<'config> SolidityParser<'config> {
    pub fn new(config: &'config SolidityLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<SolidityLanguage> for SolidityParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SolidityLanguage>) -> ParseOutput<'a, SolidityLanguage> {
        let lexer = SolidityLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
