pub mod element_type;

use crate::{language::TexLanguage, lexer::TexLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, TexLanguage, S>;

pub struct TexParser<'config> {
    pub(crate) config: &'config TexLanguage,
}

impl<'config> TexParser<'config> {
    pub fn new(config: &'config TexLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TexLanguage> for TexParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TexLanguage>) -> ParseOutput<'a, TexLanguage> {
        let lexer = TexLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
