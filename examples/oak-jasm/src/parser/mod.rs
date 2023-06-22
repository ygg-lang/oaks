use crate::{language::JasmLanguage, lexer::JasmLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, JasmLanguage, S>;

pub struct JasmParser<'config> {
    pub(crate) config: &'config JasmLanguage,
}

impl<'config> JasmParser<'config> {
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<JasmLanguage> for JasmParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JasmLanguage>) -> ParseOutput<'a, JasmLanguage> {
        let lexer = JasmLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
