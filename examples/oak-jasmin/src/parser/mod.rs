pub mod element_type;

use crate::{language::JasminLanguage, lexer::JasminLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_root;
mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, JasminLanguage, S>;

pub struct JasminParser<'config> {
    pub(crate) config: &'config JasminLanguage,
}

impl<'config> JasminParser<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<JasminLanguage> for JasminParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JasminLanguage>) -> ParseOutput<'a, JasminLanguage> {
        let lexer = JasminLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
