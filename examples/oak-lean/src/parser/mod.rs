/// Element types and tree structure definitions for Lean.
pub mod element_type;

use crate::{language::LeanLanguage, lexer::LeanLexer, parser::element_type::LeanElementType};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, LeanLanguage, S>;

/// Parser implementation for the Lean language.
pub struct LeanParser<'config> {
    pub(crate) config: &'config LeanLanguage,
}

impl<'config> LeanParser<'config> {
    /// Creates a new instance of the Lean parser with the given configuration.
    pub fn new(config: &'config LeanLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<LeanLanguage> for LeanParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LeanLanguage>) -> ParseOutput<'a, LeanLanguage> {
        let lexer = LeanLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, LeanElementType::Root))
        })
    }
}
