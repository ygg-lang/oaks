pub mod element_type;

use crate::{language::TailwindLanguage, lexer::TailwindLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, TailwindLanguage, S>;

/// Parser for the Tailwind language.
pub struct TailwindParser<'config> {
    pub(crate) config: &'config TailwindLanguage,
}

impl<'config> TailwindParser<'config> {
    /// Creates a new `TailwindParser` with the given configuration.
    pub fn new(config: &'config TailwindLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TailwindLanguage> for TailwindParser<'config> {
    /// Parses the source text into a Tailwind syntax tree.
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TailwindLanguage>) -> ParseOutput<'a, TailwindLanguage> {
        let lexer = TailwindLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
