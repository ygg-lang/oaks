pub mod element_type;
pub use element_type::ClojureElementType;

use crate::language::ClojureLanguage;
use oak_core::{
    parser::{Parser, ParserState},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, ClojureLanguage, S>;

pub struct ClojureParser<'config> {
    pub(crate) _config: &'config ClojureLanguage,
}

impl<'config> ClojureParser<'config> {
    pub fn new(config: &'config ClojureLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<ClojureLanguage> for ClojureParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<ClojureLanguage>) -> oak_core::ParseOutput<'a, ClojureLanguage> {
        let lexer = crate::lexer::ClojureLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
