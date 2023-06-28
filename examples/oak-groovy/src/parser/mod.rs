#![doc = include_str!("readme.md")]

pub mod element_type;

use crate::{language::GroovyLanguage, parser::element_type::GroovyElementType};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Type alias for the parser state.
pub(crate) type State<'a, S> = ParserState<'a, GroovyLanguage, S>;

/// Parser for Groovy source code.
pub struct GroovyParser<'config> {
    pub(crate) config: &'config GroovyLanguage,
}

impl<'config> GroovyParser<'config> {
    /// Creates a new `GroovyParser` with the given configuration.
    pub fn new(config: &'config GroovyLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<GroovyLanguage> for GroovyParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GroovyLanguage>) -> ParseOutput<'a, GroovyLanguage> {
        let lexer = crate::lexer::GroovyLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance()
            }

            Ok(state.finish_at(checkpoint, GroovyElementType::Root))
        })
    }
}
