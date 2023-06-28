#![doc = include_str!("readme.md")]

/// Element types for the DOT language syntax tree.
pub mod element_type;

use crate::{language::DotLanguage, lexer::DotLexer};
use oak_core::{
    parser::{ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, DotLanguage, S>;

/// A parser for the DOT language.
pub struct DotParser<'config> {
    pub(crate) config: &'config DotLanguage,
}

impl<'config> DotParser<'config> {
    /// Creates a new `DotParser` with the given configuration.
    pub fn new(config: &'config DotLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DotLanguage> for DotParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DotLanguage>) -> ParseOutput<'a, DotLanguage> {
        let lexer = DotLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                state.bump();
            }
            Ok(state.finish_at(checkpoint, element_type::DotElementType::Root))
        })
    }
}
