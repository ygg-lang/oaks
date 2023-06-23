pub mod element_type;

use crate::{language::TwigLanguage, lexer::TwigLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, TwigLanguage, S>;

pub struct TwigParser<'config> {
    pub(crate) config: &'config TwigLanguage,
}

impl<'config> TwigParser<'config> {
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TwigLanguage> for TwigParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TwigLanguage>) -> ParseOutput<'a, TwigLanguage> {
        let lexer = TwigLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
