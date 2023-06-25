//! Parser implementation for the D language.

pub mod element_type;

use crate::{language::DLanguage, lexer::DLexer};
use oak_core::{
    TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

/// The state of the D parser.
pub(crate) type State<'a, S> = ParserState<'a, DLanguage, S>;

/// A parser for the D language.
pub struct DParser<'config> {
    pub(crate) config: &'config DLanguage,
}

impl<'config> DParser<'config> {
    /// Creates a new D parser.
    pub fn new(config: &'config DLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DLanguage> for DParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DLanguage>) -> oak_core::ParseOutput<'a, DLanguage> {
        let lexer = DLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::DElementType::Root))
        })
    }
}
