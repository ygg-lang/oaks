//! Voml parser implementation.

pub mod element_type;

use crate::{language::VomlLanguage, lexer::VomlLexer};
use oak_core::{
    Source, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
};

/// A parser for the Voml language.
pub struct VomlParser<'config> {
    /// The Voml language configuration.
    pub(crate) config: &'config VomlLanguage,
}

impl<'config> VomlParser<'config> {
    /// Creates a new `VomlParser` with the given configuration.
    pub fn new(config: &'config VomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VomlLanguage> for VomlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VomlLanguage>) -> ParseOutput<'a, VomlLanguage> {
        let lexer = VomlLexer::new(self.config);
        parse_with_lexer(&lexer, source, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                state.advance();
            }
            Ok(state.finish_at(checkpoint, element_type::VomlElementType::SourceFile))
        })
    }
}
