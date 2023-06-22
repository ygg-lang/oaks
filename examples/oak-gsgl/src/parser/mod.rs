use crate::language::GsglLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, GsglLanguage, S>;

pub struct GsglParser {
    pub(crate) _config: GsglLanguage,
}

impl GsglParser {
    pub fn new(config: GsglLanguage) -> Self {
        Self { _config: config }
    }
}

impl Parser<GsglLanguage> for GsglParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GsglLanguage>) -> ParseOutput<'a, GsglLanguage> {
        let lexer = crate::lexer::GsglLexer::new(&self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
