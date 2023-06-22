use crate::{language::DHallLanguage, lexer::DHallLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DHallLanguage, S>;

pub struct DHallParser<'config> {
    pub(crate) _config: &'config DHallLanguage,
}

impl<'config> DHallParser<'config> {
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<DHallLanguage> for DHallParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DHallLanguage>) -> ParseOutput<'a, DHallLanguage> {
        let lexer = DHallLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
