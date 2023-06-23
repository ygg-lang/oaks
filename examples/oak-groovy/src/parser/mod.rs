pub mod element_type;

use crate::language::GroovyLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, GroovyLanguage, S>;

pub struct GroovyParser<'config> {
    pub(crate) _config: &'config GroovyLanguage,
}

impl<'config> GroovyParser<'config> {
    pub fn new(config: &'config GroovyLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<GroovyLanguage> for GroovyParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GroovyLanguage>) -> ParseOutput<'a, GroovyLanguage> {
        let lexer = crate::lexer::GroovyLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
