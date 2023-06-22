use crate::{language::CoqLanguage, lexer::CoqLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser},
    source::{Source, TextEdit},
};

mod parse;

/// Coq parser implementation.
pub struct CoqParser<'config> {
    #[allow(dead_code)]
    pub(crate) config: &'config CoqLanguage,
}

impl<'config> CoqParser<'config> {
    /// Creates a new Coq parser with the given language configuration.
    pub fn new(config: &'config CoqLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<CoqLanguage> for CoqParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CoqLanguage>) -> ParseOutput<'a, CoqLanguage> {
        let lexer = CoqLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
