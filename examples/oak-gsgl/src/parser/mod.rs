use crate::language::GsglLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, GsglLanguage, S>;

pub struct GsglParser<'config> {
    pub(crate) _config: &'config GsglLanguage,
}

impl<'config> GsglParser<'config> {
    pub fn new(config: &'config GsglLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<GsglLanguage> for GsglParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GsglLanguage>) -> ParseOutput<'a, GsglLanguage> {
        let lexer = crate::lexer::GsglLexer::new();
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
