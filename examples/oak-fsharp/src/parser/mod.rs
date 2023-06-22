use crate::{language::FSharpLanguage, lexer::FSharpLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, FSharpLanguage, S>;

pub struct FSharpParser<'config> {
    pub(crate) _config: &'config FSharpLanguage,
}

impl<'config> FSharpParser<'config> {
    pub fn new(config: &'config FSharpLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<FSharpLanguage> for FSharpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FSharpLanguage>) -> ParseOutput<'a, FSharpLanguage> {
        let lexer = FSharpLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
