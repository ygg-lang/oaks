pub mod element_type;

use crate::{language::ScssLanguage, lexer::ScssLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, ScssLanguage, S>;

/// Parser for the SCSS language.
pub struct ScssParser<'config> {
    pub(crate) config: &'config ScssLanguage,
}

impl<'config> ScssParser<'config> {
    /// Creates a new `ScssParser` with the given configuration.
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<ScssLanguage> for ScssParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ScssLanguage>) -> ParseOutput<'a, ScssLanguage> {
        let lexer = ScssLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
